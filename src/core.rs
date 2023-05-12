use std::{fmt::Debug, ops::Deref};

use crate::combinators::{
    ParserAvoid, ParserCatenate, ParserChoice, ParserEnsure, ParserLabeled, ParserLog, ParserMaybe,
    ParserMsg, ParserPlus, ParserSeq, ParserStar,
};

pub trait Identifier: Copy + 'static {}

pub enum ErrorBacktrace<T: Identifier> {
    Node { identifier: T, next: Box<Self> },
    Empty,
}
impl<'a, T: Identifier + std::fmt::Debug> std::fmt::Debug for ErrorBacktrace<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Node {
                identifier: i,
                next: n,
            } => {
                write!(f, "{:?}", i)?;
                match n.as_ref() {
                    Self::Empty => Ok(()),
                    n => write!(f, "\n⤷\t{:?}", n),
                }
            }
            Self::Empty => Ok(()),
        }
    }
}

pub struct ParseError<'a, T: Identifier> {
    pub(crate) location: &'a str,
    pub(crate) expected: &'a dyn Parser<T>,
    pub(crate) backtrace: ErrorBacktrace<T>,
    pub(crate) msg: Option<&'static str>,
}
impl<T: Identifier> std::fmt::Display for ParseError<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parsing error occured in string:\n{}", self.location)?;
        match self.msg {
            Some(s) => write!(f, "\nNote:\n{}", s),
            None => write!(
                f,
                "\nNote:\nSee this error's debug print for more information!"
            ),
        }
    }
}
impl<T: Identifier + std::fmt::Debug> std::fmt::Debug for ParseError<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Parsing error occured in string:\n{}\nIn parser:\n{:?}\nfollowing this backtrace:{:?}",
            self.location, self.expected, /*ill think about it*/ self.backtrace
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum NonTerminal<'a, T: Identifier> {
    Node { identifier: T, children: Box<Self> },
    Congregate(Vec<Self>),
    // this imposes that StrState lives as long
    // as this nonterminal
    Leaf(&'a str),
    Empty,
}

#[derive(Clone, Copy)]
pub struct StrState<'a> {
    pub string: &'a str,
    pub(crate) head: usize,
    column: usize,
    line: usize,
}
impl<'a> StrState<'a> {
    pub fn new(s: &'a str) -> Self {
        StrState {
            string: s,
            head: 0,
            column: 0,
            line: 0,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.string.len() == self.head
    }
    pub fn line_of(&self) -> &'a str {
        self.string.lines().nth(self.line).unwrap()
    }
    pub fn advance(mut self, n: usize) -> Self {
        assert!(
            self.string.len() > self.head,
            "Called method advance on StrState when it's already empty!"
        );
        for c in self.string[self.head..self.head + n].chars() {
            if c == '\n' {
                self.column = 0;
                self.line += 1;
            } else {
                self.column += 1;
            }
        }
        self.head += n;
        return self;
    }
}
impl<'a> Deref for StrState<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.string[self.head..]
    }
}
impl Debug for StrState<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.deref())
    }
}

pub trait Parser<T: Identifier>: Debug {
    fn run<'a>(
        &'a self,
        input: StrState<'a>,
    ) -> Result<(NonTerminal<'a, T>, StrState<'a>), (ParseError<'a, T>, StrState<'a>)>;
    fn to_dyn(self: Box<Self>) -> Box<dyn Parser<T>>;
    fn msg(self: Box<Self>, msg: &'static str) -> Box<dyn Parser<T>> {
        Box::new(ParserMsg {
            recipe: self.to_dyn(),
            msg,
        })
    }
    fn label(self: Box<Self>, ident: T) -> Box<dyn Parser<T>> {
        Box::new(ParserLabeled {
            recipe: self.to_dyn(),
            label: ident,
        })
    }
    fn seq(self: Box<Self>, p: Box<dyn Parser<T>>) -> Box<dyn Parser<T>> {
        Box::new(ParserSeq {
            recipe: vec![self.to_dyn(), p],
        })
    }
    fn or(self: Box<Self>, p: Box<dyn Parser<T>>) -> Box<dyn Parser<T>> {
        Box::new(ParserChoice {
            recipe: vec![self.to_dyn(), p],
        })
    }
    fn atleast_once(self: Box<Self>) -> Box<dyn Parser<T>> {
        Box::new(ParserPlus {
            recipe: self.to_dyn(),
        })
    }
    fn multiple(self: Box<Self>) -> Box<dyn Parser<T>> {
        Box::new(ParserStar {
            recipe: self.to_dyn(),
        })
    }
    fn maybe(self: Box<Self>) -> Box<dyn Parser<T>> {
        Box::new(ParserMaybe {
            recipe: self.to_dyn(),
        })
    }
    fn ensure(self: Box<Self>) -> Box<dyn Parser<T>> {
        Box::new(ParserEnsure {
            recipe: self.to_dyn(),
        })
    }
    fn avoid(self: Box<Self>) -> Box<dyn Parser<T>> {
        Box::new(ParserAvoid {
            recipe: self.to_dyn(),
        })
    }
    fn catenate(self: Box<Self>) -> Box<dyn Parser<T>> {
        Box::new(ParserCatenate {
            recipe: self.to_dyn(),
        })
    }
    fn log(
        self: Box<Self>,
        logger: Box<
            dyn Fn(
                &Result<(NonTerminal<'_, T>, StrState<'_>), (ParseError<'_, T>, StrState<'_>)>,
            ) -> (),
        >,
    ) -> Box<dyn Parser<T>> {
        Box::new(ParserLog {
            recipe: self.to_dyn(),
            logger,
        })
    }
}
