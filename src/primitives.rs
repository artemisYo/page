use crate::core::{ErrorBacktrace, Identifier, NonTerminal, ParseError, Parser, StrState};

pub fn pchar<T: Identifier>(c: char) -> Box<dyn Parser<T>> {
    Box::new(ParserChar(c))
}
pub struct ParserChar(char);
impl<T: Identifier> Parser<T> for ParserChar {
    fn to_dyn(self: Box<Self>) -> Box<dyn Parser<T>> {
        self
    }
    fn run<'a>(
        &'a self,
        input: StrState<'a>,
    ) -> Result<(NonTerminal<'a, T>, StrState<'a>), (ParseError<'a, T>, StrState<'a>)> {
        if input.string.starts_with(self.0) {
            Ok((NonTerminal::Leaf(&input.string[0..1]), input.advance(1)))
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
impl<T: Identifier> Parser<T> for ParserStr {
    fn run<'a>(
        &'a self,
        input: StrState<'a>,
    ) -> Result<(NonTerminal<'a, T>, StrState<'a>), (ParseError<'a, T>, StrState<'a>)> {
        if input.string.starts_with(self.0) {
            Ok((
                NonTerminal::Leaf(&input.string[0..self.0.len()]),
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
impl<T: Identifier> Parser<T> for ParserPredicate {
    fn run<'a>(
        &'a self,
        input: StrState<'a>,
    ) -> Result<(NonTerminal<'a, T>, StrState<'a>), (ParseError<'a, T>, StrState<'a>)> {
        let (p, l) = self.0(input.string);
        if p {
            Ok((NonTerminal::Leaf(&input.string[0..l]), input.advance(l)))
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
