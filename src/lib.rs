// parse(pattern)                     | f   :: pattern -> Parser
//               .count()             | g   :: Parser  -> Parser
// parse(pattern).count()             | g.f :: pattern -> Parser
//               .seq(parse(pattern)) | h   :: Parser  -> Parser -> Parser
// Parser      :: String -> Result
// Result      :: ParseError | ParseOutput
// ParseError  :: Err  & String
// ParseOutput :: Succ & String

// TODO:
//   1. Implement fn run on the existing parsers
//   2. Implement some basic parsers like char()
//   3. Figure out the type of field expected in
//      ParseError
//   4. Should trait Parser require Display?

pub trait Identifier {}

pub struct ErrorBacktrace<'a, T: Identifier> {
    identifier: T,
    next: Option<&'a Self>,
}
impl<'a, T: Identifier + std::fmt::Display> std::fmt::Display for ErrorBacktrace<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n", self.identifier)?;
        match self.next {
            Some(n) => write!(f, "⤷\t{}", n),
            None => Ok(()),
        }
    }
}

pub struct ParseError<'a, T: Identifier> {
    location: &'a str,
    expected: &'a dyn Parser<T>,
    backtrace: ErrorBacktrace<'a, T>,
}
impl<'a, T: Identifier + std::fmt::Display> std::fmt::Display for ParseError<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Parsing error occured in string:\n{}\nIn parser:\n{}\nfollowing this backtrace:{}",
            self.location, "todo!", /*self.expected /*ill think about it*/ */ self.backtrace
        )
    }
}

pub enum NonTerminal<T: Identifier> {
    Node { identifier: T, children: Vec<Self> },
    // this imposes that StrState lives as long
    // as this nonterminal
    Leaf(&'static str),
}

pub struct StrState {
    string: String,
    head: usize,
    column: usize,
    line: usize,
}

pub trait Parser<T: Identifier + 'static> {
    fn run(
        &self,
        input: StrState,
    ) -> Result<(NonTerminal<T>, StrState), (ParseError<'_, T>, StrState)>;
    fn to_dyn(self: Box<Self>) -> Box<dyn Parser<T>>;
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
}

pub struct ParserLabeled<T> {
    recipe: Box<dyn Parser<T>>,
    label: T,
}
impl<T: Identifier + 'static> Parser<T> for ParserLabeled<T> {
    fn run(
        &self,
        input: StrState,
    ) -> Result<(NonTerminal<T>, StrState), (ParseError<'_, T>, StrState)> {
        todo!()
    }
    fn to_dyn(self: Box<Self>) -> Box<dyn Parser<T>> {
        return self;
    }
    fn label(mut self: Box<Self>, ident: T) -> Box<dyn Parser<T>> {
        self.label = ident;
        return self;
    }
}

pub struct ParserSeq<T> {
    recipe: Vec<Box<dyn Parser<T>>>,
}
impl<T: Identifier + 'static> Parser<T> for ParserSeq<T> {
    fn run(
        &self,
        input: StrState,
    ) -> Result<(NonTerminal<T>, StrState), (ParseError<'_, T>, StrState)> {
        todo!()
    }
    fn to_dyn(self: Box<Self>) -> Box<dyn Parser<T>> {
        return self;
    }
    fn seq(mut self: Box<Self>, p: Box<dyn Parser<T>>) -> Box<dyn Parser<T>> {
        self.recipe.push(p);
        return self;
    }
}

pub struct ParserChoice<T> {
    recipe: Vec<Box<dyn Parser<T>>>,
}
impl<T: Identifier + 'static> Parser<T> for ParserChoice<T> {
    fn run(
        &self,
        input: StrState,
    ) -> Result<(NonTerminal<T>, StrState), (ParseError<'_, T>, StrState)> {
        todo!()
    }
    fn to_dyn(self: Box<Self>) -> Box<dyn Parser<T>> {
        return self;
    }
    fn or(mut self: Box<Self>, p: Box<dyn Parser<T>>) -> Box<dyn Parser<T>> {
        self.recipe.push(p);
        return self;
    }
}

pub struct ParserPlus<T> {
    recipe: Box<dyn Parser<T>>,
}
impl<T: Identifier + 'static> Parser<T> for ParserPlus<T> {
    fn run(
        &self,
        input: StrState,
    ) -> Result<(NonTerminal<T>, StrState), (ParseError<'_, T>, StrState)> {
        todo!()
    }
    fn to_dyn(self: Box<Self>) -> Box<dyn Parser<T>> {
        return self;
    }
    fn atleast_once(self: Box<Self>) -> Box<dyn Parser<T>> {
        return self;
    }
}

pub struct ParserStar<T> {
    recipe: Box<dyn Parser<T>>,
}
impl<T: Identifier + 'static> Parser<T> for ParserStar<T> {
    fn run(
        &self,
        input: StrState,
    ) -> Result<(NonTerminal<T>, StrState), (ParseError<'_, T>, StrState)> {
        todo!()
    }
    fn to_dyn(self: Box<Self>) -> Box<dyn Parser<T>> {
        return self;
    }
    fn multiple(self: Box<Self>) -> Box<dyn Parser<T>> {
        return self;
    }
}

pub struct ParserMaybe<T> {
    recipe: Box<dyn Parser<T>>,
}
impl<T: Identifier + 'static> Parser<T> for ParserMaybe<T> {
    fn run(
        &self,
        input: StrState,
    ) -> Result<(NonTerminal<T>, StrState), (ParseError<'_, T>, StrState)> {
        todo!()
    }
    fn to_dyn(self: Box<Self>) -> Box<dyn Parser<T>> {
        return self;
    }
    fn maybe(self: Box<Self>) -> Box<dyn Parser<T>> {
        return self;
    }
}

pub struct ParserEnsure<T> {
    recipe: Box<dyn Parser<T>>,
}
impl<T: Identifier + 'static> Parser<T> for ParserEnsure<T> {
    fn run(
        &self,
        input: StrState,
    ) -> Result<(NonTerminal<T>, StrState), (ParseError<'_, T>, StrState)> {
        todo!()
    }
    fn to_dyn(self: Box<Self>) -> Box<dyn Parser<T>> {
        return self;
    }
    fn ensure(self: Box<Self>) -> Box<dyn Parser<T>> {
        return self;
    }
}

pub struct ParserAvoid<T> {
    recipe: Box<dyn Parser<T>>,
}
impl<T: Identifier + 'static> Parser<T> for ParserAvoid<T> {
    fn run(
        &self,
        input: StrState,
    ) -> Result<(NonTerminal<T>, StrState), (ParseError<'_, T>, StrState)> {
        todo!()
    }
    fn to_dyn(self: Box<Self>) -> Box<dyn Parser<T>> {
        return self;
    }
    fn avoid(self: Box<Self>) -> Box<dyn Parser<T>> {
        return self;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
