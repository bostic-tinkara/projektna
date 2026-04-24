//Trikotnik deluje kot spica: če se player zadane vanj, takoj umre.

use super::Ovira;

pub struct Trikotnik {
    pub visina: u64,
    pub sirina: u64,
}

impl Ovira for Trikotnik { 
    fn new() -> Self {
        Trikotnik { visina: 2, sirina: 2 }
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
    //grafika kasneje 
    
    fn stolpen(&self) -> bool {
        false
    }

    fn naredi_stolp(visina: u64) -> Option<Box<Self>> {
        None
    }
}