mod drain;
pub mod joint;

use crate::filter::Filter;

use self::{
    drain::Drain,
    joint::{Joint, Leak},
};

pub trait Pipe<T> {
    fn pass(&mut self, x: T) -> Option<T>;
}

impl<F, T> Pipe<T> for F
where
    F: FnMut(T) -> (),
{
    fn pass(&mut self, x: T) -> Option<T> {
        (self)(x);
        None
    }
}

pub fn drain<F, T, X>(filter: F, pipe: T) -> Joint<Drain<F, T, X>, Leak, X>
where
    F: Filter<X>,
    T: Pipe<X>,
{
    Joint::new(Drain::new(filter, pipe), Leak)
}
