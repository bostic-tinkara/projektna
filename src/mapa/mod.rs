use crate::{ovire::*, player::*};

enum Ovire {
    Kvadrat,
    Trikotnik,
}

pub struct Mapa {
    stopnja: Stopnja,
    poligon: Vec<(f64, Ovire)>,
}

enum Stopnja {
    Beginner,
}

impl Mapa {
    fn new() -> Self {
        Mapa {
            stopnja: Stopnja::Beginner,
            poligon: vec![]
        }
        
    }
}


// work in progress
// struct OviraMapa {
//     pozicija: f64,
//     lik: Ovire,
// }
