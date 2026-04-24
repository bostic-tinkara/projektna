#[cfg(tests)]
mod tests {
    use crate::ovire::*;

    #[test]
    fn test_ali_lahko_naredi_stolp() {
        assert_eq!(Kvadrat.stolpen(), true);
        assert_eq!(Trikotnik.stolpen(), false);
    }
}
