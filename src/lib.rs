pub mod combinators;
pub mod core;
pub mod primitives;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pchar_works() {
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
    fn pstr_works() {
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
    fn ppred_works() {
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
    fn star_works() {
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
            ]));
        } else {
            panic!("Parser failed!");
        }
    }
}
