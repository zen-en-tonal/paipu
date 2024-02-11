use super::Pipe;
use crate::filter::Filter;
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Drain<F, P, X>(F, P, PhantomData<X>);

impl<F, P, X> Drain<F, P, X> {
    pub(super) fn new(f: F, p: P) -> Self {
        Drain(f, p, PhantomData)
    }
}

impl<F, P, X> Filter<X> for Drain<F, P, X>
where
    F: Filter<X>,
{
    fn can_pass(&self, x: &X) -> bool {
        self.0.can_pass(x)
    }
}

impl<F, P, X> Pipe<X> for Drain<F, P, X>
where
    F: Filter<X>,
    P: Pipe<X>,
{
    fn pass(&mut self, x: X) -> Option<X> {
        if self.0.can_pass(&x) {
            self.1.pass(x)
        } else {
            None
        }
    }
}
