use super::{drain::Drain, Pipe};
use crate::filter::Filter;
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Joint<D, P, X>(D, P, PhantomData<X>);

impl<D, P, X> Joint<D, P, X> {
    pub(super) fn new(d: D, p: P) -> Self {
        Joint(d, p, PhantomData)
    }
}

impl<F, T, X> Pipe<X> for Joint<F, T, X>
where
    F: Filter<X> + Pipe<X>,
    T: Pipe<X>,
{
    fn pass(&mut self, x: X) -> Option<X> {
        if self.0.can_pass(&x) {
            self.0.pass(x)
        } else {
            self.1.pass(x)
        }
    }
}

impl<F, T, X> Filter<X> for Joint<F, T, X>
where
    F: Filter<X>,
{
    fn can_pass(&self, x: &X) -> bool {
        self.0.can_pass(x)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Leak;

impl<X> Pipe<X> for Leak {
    fn pass(&mut self, x: X) -> Option<X> {
        Some(x)
    }
}

impl<F, T, X> Joint<F, T, X>
where
    F: Filter<X> + Pipe<X>,
{
    pub fn drain<A, P>(self, f: A, p: P) -> Joint<Drain<A, P, X>, Self, X>
    where
        A: Filter<X>,
        P: Pipe<X>,
    {
        Joint(Drain::new(f, p), self, PhantomData)
    }
}
