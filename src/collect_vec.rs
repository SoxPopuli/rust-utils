pub trait CollectVec: Iterator + Sized {
    fn collect_vec(self) -> Vec<Self::Item> {
        self.collect()
    }
}
impl<T> CollectVec for T where T: Iterator {}

pub trait CollectVecResult: Iterator + Sized {
    type Error;
    type Inner;
    fn collect_vec_result(self) -> Result<Vec<Self::Inner>, Self::Error>;
}
impl<T, I, E> CollectVecResult for T
where
    T: Iterator<Item = Result<I, E>>,
{
    type Inner = I;
    type Error = E;
    fn collect_vec_result(self) -> Result<Vec<Self::Inner>, Self::Error> {
        self.collect()
    }
}

pub trait CollectVecOption: Iterator + Sized {
    type Inner;
    fn collect_vec_option(self) -> Option<Vec<Self::Inner>>;
}

impl<T, I> CollectVecOption for T
where
    T: Iterator<Item = Option<I>>,
{
    type Inner = I;
    fn collect_vec_option(self) -> Option<Vec<Self::Inner>> {
        self.collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collect_test() {
        let iter = 0..10;
        let data = iter.collect_vec();
        assert_eq!(data, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn collect_vec_result_test() {
        let ok_seq: [Result<i32, ()>; 4] = [Ok(1), Ok(2), Ok(3), Ok(4)];

        assert_eq!(
            ok_seq.into_iter().collect_vec_result().unwrap(),
            [1, 2, 3, 4]
        );

        let err_seq = [Ok(1), Ok(2), Err("error"), Ok(4)];

        assert_eq!(err_seq.into_iter().collect_vec_result(), Err("error"));
    }

    #[test]
    fn collect_vec_option_test() {
        let some_seq = [Some(1), Some(2), Some(3), Some(4)];
        assert_eq!(
            some_seq.into_iter().collect_vec_option(),
            Some(vec![1, 2, 3, 4])
        );

        let none_seq = [ Some(1), Some(2), None, Some(4) ];
        assert_eq!(
            none_seq.into_iter().collect_vec_option(),
            None
        );
    }
}
