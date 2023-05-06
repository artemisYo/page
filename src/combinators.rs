// parse(pattern)                     | f   :: pattern -> Parser
//               .count()             | g   :: Parser  -> Parser
// parse(pattern).count()             | g.f :: pattern -> Parser
//               .seq(parse(pattern)) | h   :: Parser  -> Parser -> Parser
// Parser      :: String -> Result
// Result      :: ParseError | ParseOutput
// ParseError  :: Err  & String
// ParseOutput :: Succ & String

use std::fmt::Debug;

use crate::core::{ErrorBacktrace, Identifier, NonTerminal, ParseError, Parser, StrState};

pub struct ParserMsg<T> {
    pub(crate) recipe: Box<dyn Parser<T>>,
    pub(crate) msg: &'static str,
}
impl<T: Identifier> Debug for ParserMsg<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.recipe)
    }
}
impl<T: Identifier> Parser<T> for ParserMsg<T> {
    fn to_dyn(self: Box<Self>) -> Box<dyn Parser<T>> {
        self
    }
    fn run<'a>(
        &'a self,
        input: StrState<'a>,
    ) -> Result<(NonTerminal<'a, T>, StrState<'a>), (ParseError<'a, T>, StrState<'a>)> {
        match self.recipe.run(input) {
            Err((mut e, s)) => {
                if e.msg.is_none() {
                    e.msg = Some(self.msg);
                }
                return Err((e, s));
            }
            o => o,
        }
    }
    fn msg(mut self: Box<Self>, msg: &'static str) -> Box<dyn Parser<T>> {
        self.msg = msg;
        self
    }
}

pub struct ParserLabeled<T> {
    pub(crate) recipe: Box<dyn Parser<T>>,
    pub(crate) label: T,
}
impl<T: Identifier> Debug for ParserLabeled<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.recipe)
    }
}
impl<T: Identifier> Parser<T> for ParserLabeled<T> {
    fn run<'a>(
        &'a self,
        input: StrState<'a>,
    ) -> Result<(NonTerminal<'a, T>, StrState<'a>), (ParseError<'a, T>, StrState<'a>)> {
        match self.recipe.run(input) {
            Ok((n, s)) => Ok((
                NonTerminal::Node {
                    identifier: self.label,
                    children: Box::new(n),
                },
                s,
            )),
            Err((mut e, s)) => {
                let b = e.backtrace;
                e.backtrace = ErrorBacktrace::Node {
                    identifier: self.label,
                    next: Box::new(b),
                };
                Err((e, s))
            }
        }
    }
    fn to_dyn(self: Box<Self>) -> Box<dyn Parser<T>> {
        self
    }
    fn label(mut self: Box<Self>, ident: T) -> Box<dyn Parser<T>> {
        self.label = ident;
        self
    }
}

pub struct ParserSeq<T> {
    pub(crate) recipe: Vec<Box<dyn Parser<T>>>,
}
impl<T: Identifier> Debug for ParserSeq<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sequence of\n")?;
        for p in self.recipe.iter() {
            write!(f, "- {:?}", p)?;
        }
        Ok(())
    }
}
impl<T: Identifier> Parser<T> for ParserSeq<T> {
    fn run<'a>(
        &'a self,
        input: StrState<'a>,
    ) -> Result<(NonTerminal<'a, T>, StrState<'a>), (ParseError<'a, T>, StrState<'a>)> {
        let mut children = Vec::new();
        let mut head = input;
        for p in self.recipe.iter() {
            match p.run(head) {
                Ok((n, s)) => {
                    children.push(n);
                    head = s;
                }
                Err((e, _)) => {
                    return Err((e, input));
                }
            }
        }
        return Ok((NonTerminal::Congregate(children), head));
    }
    fn to_dyn(self: Box<Self>) -> Box<dyn Parser<T>> {
        self
    }
    fn seq(mut self: Box<Self>, p: Box<dyn Parser<T>>) -> Box<dyn Parser<T>> {
        self.recipe.push(p);
        self
    }
}

pub struct ParserChoice<T> {
    pub(crate) recipe: Vec<Box<dyn Parser<T>>>,
}
impl<T: Identifier> Debug for ParserChoice<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Choice between\n")?;
        for p in self.recipe.iter() {
            write!(f, "- {:?}", p)?;
        }
        Ok(())
    }
}
impl<T: Identifier> Parser<T> for ParserChoice<T> {
    fn run<'a>(
        &'a self,
        input: StrState<'a>,
    ) -> Result<(NonTerminal<'a, T>, StrState<'a>), (ParseError<'a, T>, StrState<'a>)> {
        for p in self.recipe.iter() {
            match p.run(input) {
                Err(_) => {}
                e => {
                    return e;
                }
            }
        }
        return Err((
            ParseError {
                location: input.line_of(),
                expected: self,
                backtrace: ErrorBacktrace::Empty,
                msg: None,
            },
            input,
        ));
    }
    fn to_dyn(self: Box<Self>) -> Box<dyn Parser<T>> {
        self
    }
    fn or(mut self: Box<Self>, p: Box<dyn Parser<T>>) -> Box<dyn Parser<T>> {
        self.recipe.push(p);
        self
    }
}

// TODO: from here onwards, define Debug

pub struct ParserPlus<T> {
    pub(crate) recipe: Box<dyn Parser<T>>,
}
impl<T: Identifier> Parser<T> for ParserPlus<T> {
    fn run<'a>(
        &'a self,
        input: StrState<'a>,
    ) -> Result<(NonTerminal<'a, T>, StrState<'a>), (ParseError<'a, T>, StrState<'a>)> {
        let first_res = self.recipe.run(input);
        if let Ok((n, mut head)) = first_res {
            let mut children = vec![n];
            while let Ok((n, s)) = self.recipe.run(head) {
                children.push(n);
                head = s;
            }
            return Ok((NonTerminal::Congregate(children), head));
        } else {
            return first_res;
        }
    }
    fn to_dyn(self: Box<Self>) -> Box<dyn Parser<T>> {
        self
    }
    fn atleast_once(self: Box<Self>) -> Box<dyn Parser<T>> {
        self
    }
}

pub struct ParserStar<T> {
    pub(crate) recipe: Box<dyn Parser<T>>,
}
impl<T: Identifier> Parser<T> for ParserStar<T> {
    fn run<'a>(
        &'a self,
        input: StrState<'a>,
    ) -> Result<(NonTerminal<'a, T>, StrState<'a>), (ParseError<'a, T>, StrState<'a>)> {
        let mut children = Vec::new();
        let mut head = input;
        while let Ok((n, s)) = self.recipe.run(head) {
            children.push(n);
            head = s;
        }
        if children.is_empty() {
            return Ok((NonTerminal::Empty, head));
        }
        return Ok((NonTerminal::Congregate(children), head));
    }
    fn to_dyn(self: Box<Self>) -> Box<dyn Parser<T>> {
        self
    }
    fn multiple(self: Box<Self>) -> Box<dyn Parser<T>> {
        self
    }
}

pub struct ParserMaybe<T> {
    pub(crate) recipe: Box<dyn Parser<T>>,
}
impl<T: Identifier> Parser<T> for ParserMaybe<T> {
    fn run<'a>(
        &'a self,
        input: StrState<'a>,
    ) -> Result<(NonTerminal<'a, T>, StrState<'a>), (ParseError<'a, T>, StrState<'a>)> {
        match self.recipe.run(input) {
            Err(_) => Ok((NonTerminal::Empty, input)),
            o => o,
        }
    }
    fn to_dyn(self: Box<Self>) -> Box<dyn Parser<T>> {
        self
    }
    fn maybe(self: Box<Self>) -> Box<dyn Parser<T>> {
        self
    }
}

pub struct ParserEnsure<T> {
    pub(crate) recipe: Box<dyn Parser<T>>,
}
impl<T: Identifier> Parser<T> for ParserEnsure<T> {
    fn run<'a>(
        &'a self,
        input: StrState<'a>,
    ) -> Result<(NonTerminal<'a, T>, StrState<'a>), (ParseError<'a, T>, StrState<'a>)> {
        match self.recipe.run(input) {
            Ok(_) => Ok((NonTerminal::Empty, input)),
            e => e,
        }
    }
    fn to_dyn(self: Box<Self>) -> Box<dyn Parser<T>> {
        self
    }
    fn ensure(self: Box<Self>) -> Box<dyn Parser<T>> {
        self
    }
}

pub struct ParserAvoid<T> {
    pub(crate) recipe: Box<dyn Parser<T>>,
}
impl<T: Identifier> Parser<T> for ParserAvoid<T> {
    fn run<'a>(
        &'a self,
        input: StrState<'a>,
    ) -> Result<(NonTerminal<'a, T>, StrState<'a>), (ParseError<'a, T>, StrState<'a>)> {
        match self.recipe.run(input) {
            Ok(_) => Err((
                ParseError {
                    location: input.line_of(),
                    expected: self.recipe.as_ref(),
                    backtrace: ErrorBacktrace::Empty,
                    msg: None,
                },
                input,
            )),
            Err(_) => Ok((NonTerminal::Empty, input)),
        }
    }
    fn to_dyn(self: Box<Self>) -> Box<dyn Parser<T>> {
        self
    }
    fn avoid(self: Box<Self>) -> Box<dyn Parser<T>> {
        self
    }
}
