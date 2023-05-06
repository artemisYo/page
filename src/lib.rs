pub mod combinators;
pub mod core;
pub mod primitives;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pchar_works() {
        let p = primitives::pchar('c');
        let s = core::StrState::new("c");
        p.run(s);
        todo!();
    }
}
