mod kvadrat;

pub use kvadrat::*;

pub trait Ovira<T> {
    fn name(&self) -> &str;
}