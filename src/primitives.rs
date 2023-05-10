use std::fmt::Debug;
use std::ops::Deref;

use crate::core::{ErrorBacktrace, Identifier, NonTerminal, ParseError, Parser, StrState};

pub fn pchar<T: Identifier>(c: char) -> Box<dyn Parser<T>> {
    Box::new(ParserChar(c))
}
pub struct ParserChar(char);
impl Debug for ParserChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse character '{}'", self.0)
    }
}
impl<T: Identifier> Parser<T> for ParserChar {
    fn to_dyn(self: Box<Self>) -> Box<dyn Parser<T>> {
        self
    }
    fn run<'a>(
        &'a self,
        input: StrState<'a>,
    ) -> Result<(NonTerminal<'a, T>, StrState<'a>), (ParseError<'a, T>, StrState<'a>)> {
        if input.is_empty() {
            return Err((
                ParseError {
                    location: input.line_of(),
                    expected: self,
                    backtrace: ErrorBacktrace::Empty,
                    msg: Some("At end of input!"),
                },
                input,
            ));
        }
        if input.deref().starts_with(self.0) {
            Ok((
                NonTerminal::Leaf(&input.string[input.head..][0..1]),
                input.advance(1),
            ))
        } else {
            Err((
                ParseError {
                    location: input.line_of(),
                    expected: self,
                    backtrace: ErrorBacktrace::Empty,
                    msg: None,
                },
                input,
            ))
        }
    }
}

pub fn pstr<T: Identifier>(s: &'static str) -> Box<dyn Parser<T>> {
    Box::new(ParserStr(s))
}
pub struct ParserStr(&'static str);
impl Debug for ParserStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse string '{}'", self.0)
    }
}
impl<T: Identifier> Parser<T> for ParserStr {
    fn run<'a>(
        &'a self,
        input: StrState<'a>,
    ) -> Result<(NonTerminal<'a, T>, StrState<'a>), (ParseError<'a, T>, StrState<'a>)> {
        if input.is_empty() {
            return Err((
                ParseError {
                    location: input.line_of(),
                    expected: self,
                    backtrace: ErrorBacktrace::Empty,
                    msg: Some("At end of input!"),
                },
                input,
            ));
        }
        if input.deref().starts_with(self.0) {
            Ok((
                NonTerminal::Leaf(&input.string[input.head..][0..self.0.len()]),
                input.advance(self.0.len()),
            ))
        } else {
            Err((
                ParseError {
                    location: input.line_of(),
                    expected: self,
                    backtrace: ErrorBacktrace::Empty,
                    msg: None,
                },
                input,
            ))
        }
    }
    fn to_dyn(self: Box<Self>) -> Box<dyn Parser<T>> {
        self
    }
}

pub fn ppredicate<T: Identifier, P: Fn(&str) -> (bool, usize) + 'static>(
    p: P,
) -> Box<dyn Parser<T>> {
    Box::new(ParserPredicate(Box::new(p)))
}
pub struct ParserPredicate(Box<dyn Fn(&str) -> (bool, usize)>);
impl Debug for ParserPredicate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse accoding to predicate")
    }
}
impl<T: Identifier> Parser<T> for ParserPredicate {
    fn run<'a>(
        &'a self,
        input: StrState<'a>,
    ) -> Result<(NonTerminal<'a, T>, StrState<'a>), (ParseError<'a, T>, StrState<'a>)> {
        if input.is_empty() {
            return Err((
                ParseError {
                    location: input.line_of(),
                    expected: self,
                    backtrace: ErrorBacktrace::Empty,
                    msg: Some("At end of input!"),
                },
                input,
            ));
        }
        let (p, l) = self.0(input.deref());
        if p {
            Ok((
                NonTerminal::Leaf(&input.string[input.head..][0..l]),
                input.advance(l),
            ))
        } else {
            Err((
                ParseError {
                    location: input.line_of(),
                    expected: self,
                    backtrace: ErrorBacktrace::Empty,
                    msg: None,
                },
                input,
            ))
        }
    }
    fn to_dyn(self: Box<Self>) -> Box<dyn Parser<T>> {
        self
    }
}

pub fn pany<T: Identifier>() -> Box<dyn Parser<T>> {
    Box::new(ParserAny)
}
pub struct ParserAny;
impl Debug for ParserAny {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse any character")
    }
}
impl<T: Identifier> Parser<T> for ParserAny {
    fn run<'a>(
        &'a self,
        input: StrState<'a>,
    ) -> Result<(NonTerminal<'a, T>, StrState<'a>), (ParseError<'a, T>, StrState<'a>)> {
        if input.is_empty() {
            return Err((
                ParseError {
                    location: input.line_of(),
                    expected: self,
                    backtrace: ErrorBacktrace::Empty,
                    msg: Some("At end of input!"),
                },
                input,
            ));
        }
        return Ok((
            NonTerminal::Leaf(&input.string[input.head..][0..1]),
            input.advance(1),
        ));
    }
    fn to_dyn(self: Box<Self>) -> Box<dyn Parser<T>> {
        self
    }
}

pub fn pexcept<const X: usize, T: Identifier>(c: [char; X]) -> Box<dyn Parser<T>> {
    assert!(X > 0, "empty list for pexcept!");
    Box::new(ParserExcept { recipe: c })
}
pub struct ParserExcept<const X: usize> {
    recipe: [char; X],
}
impl<const X: usize> Debug for ParserExcept<X> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse any character except for ")?;
        for c in self.recipe.iter().take(X - 1) {
            write!(f, "{}, ", c)?;
        }
        write!(f, "{}", self.recipe.last().unwrap())
    }
}
impl<const X: usize, T: Identifier> Parser<T> for ParserExcept<X> {
    fn run<'a>(
        &'a self,
        input: StrState<'a>,
    ) -> Result<(NonTerminal<'a, T>, StrState<'a>), (ParseError<'a, T>, StrState<'a>)> {
        if input.is_empty() {
            return Err((
                ParseError {
                    location: input.line_of(),
                    expected: self,
                    backtrace: ErrorBacktrace::Empty,
                    msg: Some("At end of input!"),
                },
                input,
            ));
        }
        if input
            .string
            .chars()
            .nth(0)
            .map(|c| !self.recipe.contains(&c))
            .unwrap()
        {
            return Ok((
                NonTerminal::Leaf(&input.string[input.head..][0..1]),
                input.advance(1),
            ));
        }
        Err((
            ParseError {
                location: input.line_of(),
                expected: self,
                backtrace: ErrorBacktrace::Empty,
                msg: None,
            },
            input,
        ))
    }
    fn to_dyn(self: Box<Self>) -> Box<dyn Parser<T>> {
        self
    }
}

pub fn pin<const X: usize, T: Identifier>(c: [char; X]) -> Box<dyn Parser<T>> {
    assert!(X > 0, "empty list for pin!");
    Box::new(ParserOneOf { recipe: c })
}
pub struct ParserOneOf<const X: usize> {
    recipe: [char; X],
}
impl<const X: usize> Debug for ParserOneOf<X> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write! {f, "parser any character part of "}?;
        for c in self.recipe.iter().take(X - 1) {
            write!(f, "{}, ", c)?;
        }
        write!(f, "{}", self.recipe.last().unwrap())
    }
}
impl<const X: usize, T: Identifier> Parser<T> for ParserOneOf<X> {
    fn to_dyn(self: Box<Self>) -> Box<dyn Parser<T>> {
        self
    }
    fn run<'a>(
        &'a self,
        input: StrState<'a>,
    ) -> Result<(NonTerminal<'a, T>, StrState<'a>), (ParseError<'a, T>, StrState<'a>)> {
        if input.is_empty() {
            return Err((
                ParseError {
                    location: input.line_of(),
                    expected: self,
                    backtrace: ErrorBacktrace::Empty,
                    msg: Some("At end of input!"),
                },
                input,
            ));
        }
        if input
            .string
            .chars()
            .nth(0)
            .map(|c| self.recipe.contains(&c))
            .unwrap()
        {
            return Ok((
                NonTerminal::Leaf(&input.string[input.head..][0..1]),
                input.advance(1),
            ));
        }
        Err((
            ParseError {
                location: input.line_of(),
                expected: self,
                backtrace: ErrorBacktrace::Empty,
                msg: None,
            },
            input,
        ))
    }
}
