#![allow(non_snake_case)]
use core::cpu::Kyzen;

fn main() {
//waiting.
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn if_works(){
    let Kayn = Kyzen::new();
    assert_eq!(Kayn.register[0],0);
    assert_eq!(Kayn.register[1],0);
    assert_eq!(Kayn.register[2],0);
    }
}
