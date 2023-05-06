pub mod combinators;
pub mod core;
pub mod primitives;

#[cfg(test)]
mod tests {
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
        let p = primitives::ppredicate::<I, _>(
            |c| (
                ['f', 'u', 'c', 'k']
                    .contains(
                        &c.chars()
                         .nth(0)
                         .unwrap()),
                        1)
            );
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
        let p = primitives::ppredicate::<I, _>(
            |c| (
                ['f', 'u', 'c', 'k']
                    .contains(
                        &c.chars()
                         .nth(0)
                         .unwrap()),
                        1)
        ).multiple();
        let s = core::StrState::new("cukf k");
        if let Ok((r, s)) = p.run(s) {
            assert!(s.deref() == " k", "s.deref() was: {}\n", s.deref());
            assert!(r == core::NonTerminal::<I>::Congregate(vec![
                core::NonTerminal::<I>::Leaf("c"),
                core::NonTerminal::<I>::Leaf("u"),
                core::NonTerminal::<I>::Leaf("k"),
                core::NonTerminal::<I>::Leaf("f"),
            ]), "r was: {:?}", r);
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
        let p = primitives::ppredicate::<I, _>(
            |c| (
                ['f', 'u', 'c', 'k']
                    .contains(
                        &c.chars()
                         .nth(0)
                         .unwrap()),
                        1)
        ).atleast_once();
        let s = core::StrState::new("cukf k");
        if let Ok((r, s)) = p.run(s) {
            assert!(s.deref() == " k", "s.deref() was: {}\n", s.deref());
            assert!(r == core::NonTerminal::<I>::Congregate(vec![
                core::NonTerminal::<I>::Leaf("c"),
                core::NonTerminal::<I>::Leaf("u"),
                core::NonTerminal::<I>::Leaf("k"),
                core::NonTerminal::<I>::Leaf("f"),
            ]), "r was: {:?}", r);
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
        let p =
            primitives::pchar::<I>('d')
            .seq(primitives::pchar::<I>('a'))
            .seq(primitives::pchar::<I>('m'))
            .seq(primitives::pchar::<I>('n'));
        let s = core::StrState::new("amn");
        if let Ok((r, s)) = p.run(s) {
            assert!(s.is_empty());
            assert!(r == core::NonTerminal::<I>::Congregate(
                vec![
                    core::NonTerminal::<I>::Leaf("d"),
                    core::NonTerminal::<I>::Leaf("a"),
                    core::NonTerminal::<I>::Leaf("m"),
                    core::NonTerminal::<I>::Leaf("n"),
                ]
            ));
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
        let p =
            primitives::pchar::<I>('d')
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
}
