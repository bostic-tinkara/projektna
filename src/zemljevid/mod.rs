use macroquad::prelude::*;

use crate::{ovire::*, player::*};

pub enum Stopnja {
    Beginner,
}

pub struct Zemljevid {
    pub stopnja: Stopnja,
    pub poligon: Vec<(f32, Ovira)>,
    pub hitrost: f32,
}

impl Zemljevid {
    pub fn new(stopnja: Stopnja) -> Self {
        let poligon = match stopnja {
            Stopnja::Beginner => vec![
                (400.0, Ovira::nov_trikotnik(40.0, 40.0)),
                (600.0, Ovira::nov_kvadrat(50.0)),
                (800.0, Ovira::nov_kvadrat(50.0)), // Dva kvadrata skupaj
                (1000.0, Ovira::nov_trikotnik(40.0, 40.0)),
                (1200.0, Ovira::nov_trikotnik(40.0, 40.0)), // Dvojna špica                
            ],
        };

        Zemljevid {
            stopnja,
            poligon,
            hitrost: 3.,
        }
        
    }

    pub fn posodobi(&mut self) {
        for (x, _) in &mut self.poligon {
            *x -= self.hitrost;
        }
    }

    pub fn narisi(&self, tla_y: f32) {
        for (x, ovira) in &self.poligon {
            if *x > -100.0 && *x < screen_width() + 100.0 { // samo če je ovira blizu ekrana, jo narišemo
                ovira.narisi(*x, tla_y, DARKBLUE);
            }
        }
    }

}