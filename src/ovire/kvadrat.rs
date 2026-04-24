//Kvadrat deluje kot blok na katerega človeček lahko skoči in se naprej po njem premika. Če pade iz njega,
//se igrica nadaljuje. Če se zaletimo v stranico (ne skočimo nanj), se igrica ustavi.

use super::Ovira;

pub struct Kvadrat {
    pub visina: f64,
    pub sirina: f64,
}

impl Ovira for Kvadrat {
    fn new() -> Self {
        Kvadrat { visina: 1., sirina: 1. }
    }

    //fn draw(&self) {
    //    todo!()
    //}
    // grafika kasneje
    
    fn stolpen(&self) -> bool {
        true
    }
}