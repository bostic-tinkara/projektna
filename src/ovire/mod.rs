mod kvadrat;
mod trikotnik;

pub use kvadrat::*;
pub use trikotnik::*;

use crate::player;


pub trait Ovira {
    fn new() -> Self;
    fn visina(&self) -> u64;
    fn sirina(&self) -> u64;
    //fn draw(&self);
    fn stolpen(&self) -> bool; // true ce lahko na lik damo drug lik
    fn naredi_stolp(visina: u64) -> Option<Box<Self>>;
}
