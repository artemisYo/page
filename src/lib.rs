pub mod combinators;
pub mod core;
pub mod primitives;

// TODO:
//  1. Add memoization
//  2. Ignore result verb
//  3. Fix linebreak formatting in Choice and check the other parsers
//  4. Make error debug output not print linebreaks as linebreaks
//  5. Better error location reports, not only lineof

#[cfg(test)]
mod tests {
    use crate::core::StrState;

    use super::*;

    #[test]
    fn pchar_passes() {
        #[derive(PartialEq, Eq, Clone, Copy, Debug)]
        struct I;
        impl core::Identifier for I {}
        let p = primitives::pchar::<I>('c');
        let s = core::StrState::new("c");
        if let Ok((r, s)) = p.run(s) {
            assert!(s.is_empty());
            assert!(r == core::NonTerminal::<I>::Leaf("c"));
        } else {
            panic!("Parser failed!");
        }
    }
    #[test]
    fn pstr_passes() {
        use std::ops::Deref;
        #[derive(PartialEq, Eq, Clone, Copy, Debug)]
        struct I;
        impl core::Identifier for I {}
        let p = primitives::pstr::<I>("damn");
        let s = core::StrState::new("damn huh");
        if let Ok((r, s)) = p.run(s) {
            assert!(s.deref() == " huh");
            assert!(r == core::NonTerminal::<I>::Leaf("damn"));
        } else {
            panic!("Parser failed!");
        }
    }
    #[test]
    fn ppred_passes() {
        use std::ops::Deref;
        #[derive(PartialEq, Eq, Clone, Copy, Debug)]
        struct I;
        impl core::Identifier for I {}
        let p = primitives::ppredicate::<I, _>(|c| {
            (['f', 'u', 'c', 'k'].contains(&c.chars().nth(0).unwrap()), 1)
        });
        let s = core::StrState::new("cukf k");
        if let Ok((r, s)) = p.run(s) {
            assert!(s.deref() == "ukf k", "s.deref() was: {}\n", s.deref());
            assert!(r == core::NonTerminal::<I>::Leaf("c"));
        } else {
            panic!("Parser failed!");
        }
    }
    #[test]
    fn pany_passes() {
        use std::ops::Deref;
        #[derive(PartialEq, Eq, Clone, Copy, Debug)]
        struct I;
        impl core::Identifier for I {}
        let p = primitives::pany::<I>();
        let s = core::StrState::new("cukf k");
        if let Ok((r, s)) = p.run(s) {
            assert!(s.deref() == "ukf k", "s.deref() was: {}\n", s.deref());
            assert!(r == core::NonTerminal::<I>::Leaf("c"));
        } else {
            panic!("Parser failed!");
        }
    }
    #[test]
    fn star_passes() {
        use std::ops::Deref;
        #[derive(PartialEq, Eq, Clone, Copy, Debug)]
        struct I;
        impl core::Identifier for I {}
        let p = primitives::ppredicate::<I, _>(|c| {
            (['f', 'u', 'c', 'k'].contains(&c.chars().nth(0).unwrap()), 1)
        })
        .multiple();
        let s = core::StrState::new("cukf k");
        if let Ok((r, s)) = p.run(s) {
            assert!(s.deref() == " k", "s.deref() was: {}\n", s.deref());
            assert!(
                r == core::NonTerminal::<I>::Congregate(vec![
                    core::NonTerminal::<I>::Leaf("c"),
                    core::NonTerminal::<I>::Leaf("u"),
                    core::NonTerminal::<I>::Leaf("k"),
                    core::NonTerminal::<I>::Leaf("f"),
                ]),
                "r was: {:?}",
                r
            );
        } else {
            panic!("Parser failed!");
        }
    }
    #[test]
    fn plus_passes() {
        use std::ops::Deref;
        #[derive(PartialEq, Eq, Clone, Copy, Debug)]
        struct I;
        impl core::Identifier for I {}
        let p = primitives::ppredicate::<I, _>(|c| {
            (['f', 'u', 'c', 'k'].contains(&c.chars().nth(0).unwrap()), 1)
        })
        .atleast_once();
        let s = core::StrState::new("cukf k");
        if let Ok((r, s)) = p.run(s) {
            assert!(s.deref() == " k", "s.deref() was: {}\n", s.deref());
            assert!(
                r == core::NonTerminal::<I>::Congregate(vec![
                    core::NonTerminal::<I>::Leaf("c"),
                    core::NonTerminal::<I>::Leaf("u"),
                    core::NonTerminal::<I>::Leaf("k"),
                    core::NonTerminal::<I>::Leaf("f"),
                ]),
                "r was: {:?}",
                r
            );
        } else {
            panic!("Parser failed!");
        }
    }
    #[test]
    fn maybe_passes() {
        #[derive(PartialEq, Eq, Clone, Copy, Debug)]
        struct I;
        impl core::Identifier for I {}
        let p = primitives::pchar::<I>('c').maybe();
        let s = core::StrState::new("c");
        if let Ok((r, s)) = p.run(s) {
            assert!(s.is_empty());
            assert!(r == core::NonTerminal::<I>::Leaf("c"));
        } else {
            panic!("Parser failed!");
        }
    }
    #[test]
    fn seq_passes() {
        #[derive(PartialEq, Eq, Clone, Copy, Debug)]
        struct I;
        impl core::Identifier for I {}
        let p = primitives::pchar::<I>('d')
            .seq(primitives::pchar::<I>('a'))
            .seq(primitives::pchar::<I>('m'))
            .seq(primitives::pchar::<I>('n'));
        let s = core::StrState::new("damn");
        if let Ok((r, s)) = p.run(s) {
            assert!(s.is_empty());
            assert!(
                r == core::NonTerminal::<I>::Congregate(vec![
                    core::NonTerminal::<I>::Leaf("d"),
                    core::NonTerminal::<I>::Leaf("a"),
                    core::NonTerminal::<I>::Leaf("m"),
                    core::NonTerminal::<I>::Leaf("n"),
                ])
            );
        } else {
            panic!("Error: {:?}", p.run(s).unwrap_err().0);
        }
    }
    #[test]
    fn choice_passes() {
        use std::ops::Deref;
        #[derive(PartialEq, Eq, Clone, Copy, Debug)]
        struct I;
        impl core::Identifier for I {}
        let p = primitives::pchar::<I>('d')
            .or(primitives::pchar::<I>('a'))
            .or(primitives::pchar::<I>('m'))
            .or(primitives::pchar::<I>('n'));
        let s = core::StrState::new("damn");
        if let Ok((r, s)) = p.run(s) {
            assert!(s.deref() == "amn");
            assert!(r == core::NonTerminal::<I>::Leaf("d"),);
        } else {
            panic!("Parser failed!");
        }
    }
    #[test]
    fn cat_passes() {
        #[derive(PartialEq, Eq, Clone, Copy, Debug)]
        struct I;
        impl core::Identifier for I {}
        let p = primitives::pchar::<I>('d')
            .seq(primitives::pchar::<I>('a'))
            .seq(primitives::pchar::<I>('m'))
            .seq(primitives::pchar::<I>('n'))
            .catenate();
        let s = core::StrState::new("damn");
        if let Ok((r, s)) = p.run(s) {
            assert!(s.is_empty());
            assert!(r == core::NonTerminal::<I>::Leaf("damn"));
        } else {
            panic!("Parser failed!");
        }
    }
    #[test]
    fn pexcept_passes() {
        #[derive(PartialEq, Eq, Clone, Copy, Debug)]
        struct I;
        impl core::Identifier for I {}
        let p = primitives::pexcept::<4, I>(['a', 'b', 'd', 'e']);
        let s = core::StrState::new("c");
        if let Ok((r, s)) = p.run(s) {
            assert!(s.is_empty());
            assert!(r == core::NonTerminal::<I>::Leaf("c"));
        } else {
            panic!("Parser failed!");
        }
    }
    #[test]
    fn pin_passes() {
        #[derive(PartialEq, Eq, Clone, Copy, Debug)]
        struct I;
        impl core::Identifier for I {}
        let p = primitives::pin::<4, I>(['a', 'b', 'c', 'd']);
        let s = core::StrState::new("c");
        if let Ok((r, s)) = p.run(s) {
            assert!(s.is_empty());
            assert!(r == core::NonTerminal::<I>::Leaf("c"));
        } else {
            panic!("Parser failed!");
        }
    }
    #[test]
    fn combination_passes() {
        #[derive(PartialEq, Eq, Clone, Copy, Debug)]
        enum I {
            Num,
            Op,
            Add,
            Sub,
        }
        impl core::Identifier for I {}
        use primitives::*;
        // TODO: Figure out why the whitespace ting only gets an empty string
        let p = pin(['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'])
            .atleast_once()
            .catenate()
            .label(I::Num)
            .seq(pin([' ', '\n', '\t']))
            .seq(
                pchar('+')
                    .label(I::Add)
                    .or(pchar('-').label(I::Sub))
                    .label(I::Op),
            )
            .seq(pin([' ', '\n', '\t']).multiple())
            .seq(
                pin(['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'])
                    .atleast_once()
                    .catenate()
                    .label(I::Num),
            );
        let s = StrState::new("69 +   420");
        match p.run(s) {
            Ok((_, s)) => assert!(s.is_empty()),
            Err((e, _)) => panic!("Parser Failed\n[Error]:\n{:?}", e),
        }
    }
}
