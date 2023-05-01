// parse(pattern)                     | f   :: pattern -> Parser
//               .count()             | g   :: Parser  -> Parser
// parse(pattern).count()             | g.f :: pattern -> Parser
//               .seq(parse(pattern)) | h   :: Parser  -> Parser -> Parser

trait Parser {
    //fn run(&self, _: &str) -> &str;
    fn seq(self, p: Box<dyn Parser>) -> Box<dyn Parser>;
    fn or(self, p: Box<dyn Parser>) -> Box<dyn Parser>;
    fn atleast_once(self) -> Box<dyn Parser>;
    fn multiple(self) -> Box<dyn Parser>;
    fn maybe(self) -> Box<dyn Parser>;
    fn ensure(self) -> Box<dyn Parser>;
    fn avoid(self) -> Box<dyn Parser>;
}

struct ParserSeq {
    recipe: Vec<Box<dyn Parser>>,
}
impl Parser for ParserSeq {
    fn seq(mut self, p: Box<dyn Parser>) -> Box<dyn Parser> {
        self.recipe.push(p);
        return Box::new(self);
    }
    fn or(self, p: Box<dyn Parser>) -> Box<dyn Parser> {
        return Box::new(ParserChoice {
            recipe: vec![Box::new(self), p],
        });
    }
    fn atleast_once(self) -> Box<dyn Parser> {
        return Box::new(ParserPlus {
            recipe: Box::new(self),
        });
    }
    fn multiple(self) -> Box<dyn Parser> {
        return Box::new(ParserStar {
            recipe: Box::new(self),
        });
    }
    fn maybe(self) -> Box<dyn Parser> {
        return Box::new(ParserMaybe {
            recipe: Box::new(self),
        });
    }
    fn ensure(self) -> Box<dyn Parser> {
        return Box::new(ParserEnsure {
            recipe: Box::new(self),
        });
    }
    fn avoid(self) -> Box<dyn Parser> {
        return Box::new(ParserAvoid {
            recipe: Box::new(self),
        });
    }
}

struct ParserChoice {
    recipe: Vec<Box<dyn Parser>>,
}
impl Parser for ParserChoice {
    fn seq(self, p: Box<dyn Parser>) -> Box<dyn Parser> {
        return Box::new(ParserSeq {
            recipe: vec![Box::new(self), p],
        });
    }
    fn or(mut self, p: Box<dyn Parser>) -> Box<dyn Parser> {
        self.recipe.push(p);
        return Box::new(self);
    }
    fn atleast_once(self) -> Box<dyn Parser> {
        return Box::new(ParserPlus {
            recipe: Box::new(self),
        });
    }
    fn multiple(self) -> Box<dyn Parser> {
        return Box::new(ParserStar {
            recipe: Box::new(self),
        });
    }
    fn maybe(self) -> Box<dyn Parser> {
        return Box::new(ParserMaybe {
            recipe: Box::new(self),
        });
    }
    fn ensure(self) -> Box<dyn Parser> {
        return Box::new(ParserEnsure {
            recipe: Box::new(self),
        });
    }
    fn avoid(self) -> Box<dyn Parser> {
        return Box::new(ParserAvoid {
            recipe: Box::new(self),
        });
    }
}

struct ParserPlus {
    recipe: Box<dyn Parser>,
}
impl Parser for ParserPlus {
    fn seq(self, p: Box<dyn Parser>) -> Box<dyn Parser> {
        return Box::new(ParserSeq {
            recipe: vec![Box::new(self), p],
        });
    }
    fn or(self, p: Box<dyn Parser>) -> Box<dyn Parser> {
        return Box::new(ParserChoice {
            recipe: vec![Box::new(self), p],
        });
    }
    fn atleast_once(self) -> Box<dyn Parser> {
        return Box::new(self);
    }
    fn multiple(self) -> Box<dyn Parser> {
        return Box::new(ParserStar {
            recipe: Box::new(self),
        });
    }
    fn maybe(self) -> Box<dyn Parser> {
        return Box::new(ParserMaybe {
            recipe: Box::new(self),
        });
    }
    fn ensure(self) -> Box<dyn Parser> {
        return Box::new(ParserEnsure {
            recipe: Box::new(self),
        });
    }
    fn avoid(self) -> Box<dyn Parser> {
        return Box::new(ParserAvoid {
            recipe: Box::new(self),
        });
    }
}

struct ParserStar {
    recipe: Box<dyn Parser>,
}
impl Parser for ParserStar {
    fn seq(self, p: Box<dyn Parser>) -> Box<dyn Parser> {
        return Box::new(ParserSeq {
            recipe: vec![Box::new(self), p],
        });
    }
    fn or(self, p: Box<dyn Parser>) -> Box<dyn Parser> {
        return Box::new(ParserChoice {
            recipe: vec![Box::new(self), p],
        });
    }
    fn atleast_once(self) -> Box<dyn Parser> {
        return Box::new(ParserPlus {
            recipe: Box::new(self),
        });
    }
    fn multiple(self) -> Box<dyn Parser> {
        return Box::new(self);
    }
    fn maybe(self) -> Box<dyn Parser> {
        return Box::new(ParserMaybe {
            recipe: Box::new(self),
        });
    }
    fn ensure(self) -> Box<dyn Parser> {
        return Box::new(ParserEnsure {
            recipe: Box::new(self),
        });
    }
    fn avoid(self) -> Box<dyn Parser> {
        return Box::new(ParserAvoid {
            recipe: Box::new(self),
        });
    }
}

struct ParserMaybe {
    recipe: Box<dyn Parser>,
}
impl Parser for ParserMaybe {
    fn seq(self, p: Box<dyn Parser>) -> Box<dyn Parser> {
        return Box::new(ParserSeq {
            recipe: vec![Box::new(self), p],
        });
    }
    fn or(self, p: Box<dyn Parser>) -> Box<dyn Parser> {
        return Box::new(ParserChoice {
            recipe: vec![Box::new(self), p],
        });
    }
    fn atleast_once(self) -> Box<dyn Parser> {
        return Box::new(ParserPlus {
            recipe: Box::new(self),
        });
    }
    fn multiple(self) -> Box<dyn Parser> {
        return Box::new(ParserStar {
            recipe: Box::new(self),
        });
    }
    fn maybe(self) -> Box<dyn Parser> {
        return Box::new(self);
    }
    fn ensure(self) -> Box<dyn Parser> {
        return Box::new(ParserEnsure {
            recipe: Box::new(self),
        });
    }
    fn avoid(self) -> Box<dyn Parser> {
        return Box::new(ParserAvoid {
            recipe: Box::new(self),
        });
    }
}

struct ParserEnsure {
    recipe: Box<dyn Parser>,
}
impl Parser for ParserEnsure {
    fn seq(self, p: Box<dyn Parser>) -> Box<dyn Parser> {
        return Box::new(ParserSeq {
            recipe: vec![Box::new(self), p],
        });
    }
    fn or(self, p: Box<dyn Parser>) -> Box<dyn Parser> {
        return Box::new(ParserChoice {
            recipe: vec![Box::new(self), p],
        });
    }
    fn atleast_once(self) -> Box<dyn Parser> {
        return Box::new(ParserPlus {
            recipe: Box::new(self),
        });
    }
    fn multiple(self) -> Box<dyn Parser> {
        return Box::new(ParserStar {
            recipe: Box::new(self),
        });
    }
    fn maybe(self) -> Box<dyn Parser> {
        return Box::new(ParserMaybe {
            recipe: Box::new(self),
        });
    }
    fn ensure(self) -> Box<dyn Parser> {
        return Box::new(self);
    }
    fn avoid(self) -> Box<dyn Parser> {
        return Box::new(ParserAvoid {
            recipe: Box::new(self),
        });
    }
}

struct ParserAvoid {
    recipe: Box<dyn Parser>,
}
impl Parser for ParserAvoid {
    fn seq(self, p: Box<dyn Parser>) -> Box<dyn Parser> {
        return Box::new(ParserSeq {
            recipe: vec![Box::new(self), p],
        });
    }
    fn or(self, p: Box<dyn Parser>) -> Box<dyn Parser> {
        return Box::new(ParserChoice {
            recipe: vec![Box::new(self), p],
        });
    }
    fn atleast_once(self) -> Box<dyn Parser> {
        return Box::new(ParserPlus {
            recipe: Box::new(self),
        });
    }
    fn multiple(self) -> Box<dyn Parser> {
        return Box::new(ParserStar {
            recipe: Box::new(self),
        });
    }
    fn maybe(self) -> Box<dyn Parser> {
        return Box::new(ParserMaybe {
            recipe: Box::new(self),
        });
    }
    fn ensure(self) -> Box<dyn Parser> {
        return Box::new(ParserEnsure {
            recipe: Box::new(self),
        });
    }
    fn avoid(self) -> Box<dyn Parser> {
        return Box::new(self);
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
