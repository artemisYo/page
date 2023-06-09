use std::ops::Deref;

use crate::combinators::{
    ParserAvoid, ParserCatenate, ParserChoice, ParserEnsure, ParserIgnoreRes, ParserLabeled,
    ParserLog, ParserMaybe, ParserMsg, ParserPlus, ParserSeq, ParserStar,
};

pub trait Identifier: Copy + 'static {}

#[derive(Debug)]
pub enum ErrorBacktrace<T: Identifier> {
    Node { identifier: T, next: Box<Self> },
    Empty,
}
impl<T: Identifier + std::fmt::Debug> ErrorBacktrace<T> {
    pub fn info(&self) -> String {
        match self {
            Self::Node { identifier, next } => {
                format!(
                    "{:?}{}",
                    identifier,
                    match next.as_ref() {
                        Self::Empty => "".to_owned(),
                        n => format!("\n⤷\t{}", n.info()),
                    }
                )
            }
            Self::Empty => "".to_owned(),
        }
    }
}

fn point(s: &str, i: usize) -> String {
    format!("{s}\n{}^{}", " ".repeat(i), "~".repeat(s.len() - i - 1))
}

#[derive(Debug)]
pub struct ParseError<'a, T: Identifier> {
    pub(crate) location: (&'a str, usize, usize),
    pub(crate) expected: &'a dyn Parser<T>,
    pub(crate) backtrace: ErrorBacktrace<T>,
    pub(crate) msg: Option<&'static str>,
}
impl<T: Identifier> std::fmt::Display for ParseError<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}:{}]\tParsing error occured in string:\n{}",
            self.location.1,
            self.location.2,
            point(self.location.0, self.location.2)
        )?;
        match self.msg {
            Some(s) => write!(f, "\nNote:\n{}", s),
            None => write!(
                f,
                "\nNote:\nSee this error's info print for more information!"
            ),
        }
    }
}
impl<T: Identifier + std::fmt::Debug> ParseError<'_, T> {
    pub fn info(&self) -> String {
        format!(
            "[{}:{}]\tParsing error occured in string:\n{}\nIn parser:\n{:?}\nfollowing this backtrace:\n{}",
            self.location.1,
            self.location.2,
            point(self.location.0, self.location.2),
            self.expected,
            self.backtrace.info()
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
impl<'a, T: Identifier> NonTerminal<'a, T> {
    pub fn clean(self) -> Self {
        match self {
            Self::Node {
                identifier,
                children,
            } => Self::Node {
                identifier,
                children: Box::new(children.clean()),
            },
            Self::Congregate(v) => {
                let mut i = vec![];
                for c in v.into_iter().map(|n| n.clean()) {
                    match c {
                        Self::Empty => {}
                        _ => i.push(c),
                    }
                }
                if i.is_empty() {
                    Self::Empty
                } else {
                    Self::Congregate(i)
                }
            }
            e => e,
        }
    }
}

#[derive(Clone, Copy)]
pub struct StrState<'a> {
    pub string: &'a str,
    pub(crate) head: usize,
    pub(crate) column: usize,
    pub(crate) line: usize,
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
impl std::fmt::Debug for StrState<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.deref())
    }
}

pub trait Parser<T: Identifier>: std::fmt::Debug {
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
    fn ignore(self: Box<Self>) -> Box<dyn Parser<T>> {
        Box::new(ParserIgnoreRes {
            recipe: self.to_dyn(),
        })
    }
}
