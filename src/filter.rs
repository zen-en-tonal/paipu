pub trait Filter<T> {
    fn can_pass(&self, x: &T) -> bool;
}

impl<F, T> Filter<T> for F
where
    F: Fn(&T) -> bool,
{
    fn can_pass(&self, x: &T) -> bool {
        (self)(x)
    }
}
