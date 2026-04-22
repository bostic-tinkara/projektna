use super::Ovira;

pub struct Kvadrat<T> {
    ime: String,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Kvadrat<T> {}