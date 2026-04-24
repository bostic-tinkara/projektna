mod kvadrat;
mod trikotnik;

pub use kvadrat::*;
pub use trikotnik::*;

use crate::player;

pub trait Ovira {
    fn new() -> Self;
    //fn draw(&self);
    fn stolpen(&self) -> bool; // true ce lahko na lik damo drug lik
}

pub enum Ovire {
    Kvadrat,
    Trikotnik,
}