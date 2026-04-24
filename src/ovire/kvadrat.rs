// Kvadrat deluje kot blok na katerega človeček lahko skoči in se 
// naprej po njem premika. Če pade iz njega, se igrica nadaljuje. 
// Če se zaletimo v stranico (ne skočimo nanj), se igrica ustavi.

use super::Ovira;

pub struct Kvadrat {
    pub visina: u64,
    pub sirina: u64,
}

impl Ovira for Kvadrat {
    fn new() -> Self {
        Kvadrat { visina: 2, sirina: 2 }
    }

    fn visina(&self) -> u64 {
        self.visina
    }

    fn sirina(&self) -> u64 {
        self.sirina
    }

    //fn draw(&self) {
    //    todo!()
    //}
    // grafika kasneje
    
    fn stolpen(&self) -> bool { 
        // lahko na kvadrat postavimo še druge ovire, naredimo "stolp"
        true
    }

    fn naredi_stolp(visina: u64) -> Option<Box<Self>> {
        if visina == 0 || visina % 2 != 0 || visina > 10 { None } 
        else {
            Some(Box::new(Kvadrat{ visina, sirina: 2 }))
        }
    }
}