#[cfg(test)]
mod tests {
    use crate::{ovire::{Kvadrat, Ovira, Trikotnik}, player::Player};

    use super::*;

    #[test]
    fn test_kvadrat_stolpen() {
        let k = Kvadrat::new();
        assert_eq!(k.stolpen(), true);
    }

    #[test]
    fn test_trikotnik_stolpen() {
        let t = Trikotnik::new();
        assert_eq!(t.stolpen(), false);
    }

    #[test]
    fn test_lahko_preskoci_1() {
        let igralec = Player::new();
        let k1 = Kvadrat::new();

        assert_eq!(igralec.lahko_preskoci(k1), true);
    }

    #[test]
    fn test_lahko_preskoci_2() {
        let igralec = Player::new();
        let k2 = Kvadrat { visina: 8, sirina: 2 };

        assert_eq!(igralec.lahko_preskoci(k2), false);
    }

}
