mod filter;
mod pipe;

pub use filter::Filter;
pub use pipe::{drain, joint::Joint, Pipe};

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_1() {
        let mut a: Vec<i32> = vec![];
        let mut b: Vec<i32> = vec![];
        let mut c: Vec<i32> = vec![];
        let mut pipe = drain(|x: &i32| *x > 1, |x: i32| a.push(x))
            .drain(|x: &i32| *x > 5, |x: i32| b.push(x))
            .drain(|x: &i32| *x > 10, |x: i32| c.push(x));

        assert_eq!(pipe.pass(15), None);
        assert_eq!(c, [15]);
        assert_eq!(b, []);
        assert_eq!(a, []);
    }

    #[test]
    fn test_2() {
        let mut a: Vec<i32> = vec![];
        let mut b: Vec<i32> = vec![];
        let mut c: Vec<i32> = vec![];
        let mut pipe = drain(|x: &i32| *x > 1, |x: i32| a.push(x))
            .drain(|x: &i32| *x > 5, |x: i32| b.push(x))
            .drain(|x: &i32| *x > 10, |x: i32| c.push(x));

        assert_eq!(pipe.pass(8), None);
        assert_eq!(c, []);
        assert_eq!(b, [8]);
        assert_eq!(a, []);
    }

    #[test]
    fn test_3() {
        let mut a: Vec<i32> = vec![];
        let mut b: Vec<i32> = vec![];
        let mut c: Vec<i32> = vec![];
        let mut pipe = drain(|x: &i32| *x > 1, |x: i32| a.push(x))
            .drain(|x: &i32| *x > 5, |x: i32| b.push(x))
            .drain(|x: &i32| *x > 10, |x: i32| c.push(x));

        assert_eq!(pipe.pass(5), None);
        assert_eq!(c, []);
        assert_eq!(b, []);
        assert_eq!(a, [5]);
    }

    #[test]
    fn test_4() {
        let mut a: Vec<i32> = vec![];
        let mut b: Vec<i32> = vec![];
        let mut c: Vec<i32> = vec![];
        let mut pipe = drain(|x: &i32| *x > 1, |x: i32| a.push(x))
            .drain(|x: &i32| *x > 5, |x: i32| b.push(x))
            .drain(|x: &i32| *x > 10, |x: i32| c.push(x));

        assert_eq!(pipe.pass(0), Some(0));
        assert_eq!(c, []);
        assert_eq!(b, []);
        assert_eq!(a, []);
    }
}
