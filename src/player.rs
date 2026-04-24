use crate::ovire::Kvadrat;

pub enum Izgled {
    Kvadrat,
}

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub izgled: Izgled,
    
}

impl Player {
    pub fn new() -> Self { // naredi novega igralca na x, y ( (0,0) mestu)
        Player { x: 0., y: 0., izgled: Izgled::Kvadrat }
    }

    pub fn premik(&mut self, dy: f64) {
        self.y = self.y + dy;
    }
}
