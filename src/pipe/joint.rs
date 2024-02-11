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

impl<F, T, P, X> Pipe<X> for Joint<Drain<F, T, X>, P, X>
where
    F: Filter<X>,
    T: Pipe<X>,
    P: Pipe<X>,
{
    fn pass(&mut self, x: X) -> Option<X> {
        if self.0 .0.pass(&x) {
            self.0 .1.pass(x)
        } else {
            self.1.pass(x)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Leak;

impl<X> Pipe<X> for Leak {
    fn pass(&mut self, x: X) -> Option<X> {
        Some(x)
    }
}

impl<F, T, P, X> Joint<Drain<F, T, X>, P, X> {
    pub fn drain<A, Q>(self, f: A, p: Q) -> Joint<Drain<A, Q, X>, Self, X>
    where
        A: Filter<X>,
        P: Pipe<X>,
    {
        Joint(Drain::new(f, p), self, PhantomData)
    }
}
