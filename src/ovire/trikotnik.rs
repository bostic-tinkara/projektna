//Trikotnik deluje kot spica: če se player zadane vanj, takoj umre.

use super::Ovira;

pub struct Trikotnik<T> {
    x: f64,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Ovira<T> for Trikotnik<T> {
    fn name(&self) -> &str {
        todo!()
    }

    fn update(&mut self, speed: f64) {
        self.x -= speed // se premakne levo
    }

    fn draw(&self) {
        todo!()
    }

    fn se_dotika(&self, player_y: f64) -> bool {
        todo!()
    }
}