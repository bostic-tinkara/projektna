//Trikotnik deluje kot spica: če se player zadane vanj, takoj umre.

use super::Ovira;

pub struct Trikotnik {
    pub visina: f64,
    pub sirina: f64,
}

impl Ovira for Trikotnik { 
    fn new() -> Self {
        Trikotnik { visina: 1., sirina: 1. }
    }

    //fn draw(&self) {
    //    todo!()
    //}
    //grafika kasneje 
    
    fn stolpen(&self) -> bool {
        false
    }
}