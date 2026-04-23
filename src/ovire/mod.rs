mod kvadrat;
mod trikotnik;

pub use kvadrat::*;
pub use trikotnik::*;

pub trait Ovira<T> {
    fn name(&self) -> &str;
    fn update(&mut self, speed: f64);
    fn draw(&self);
    fn se_dotika(&self, player_y: f64) -> bool;
}