//Kvadrat deluje kot blok na katerega človeček lahko skoči in se naprej po njem premika. Če pade iz njega,
//se igrica nadaljuje. Če se zaletimo v stranico (ne skočimo nanj), se igrica ustavi.

use super::Ovira;

pub struct Kvadrat<T> {
    ime: String,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Ovira<T> for Kvadrat<T> {
    fn name(&self) -> &str {
        todo!()
    }

    fn update(&mut self, speed: f64) {
        todo!()
    }

    fn draw(&self) {
        todo!()
    }

    fn se_dotika(&self, player_y: f64) -> bool {
        todo!()
    }
}