pub trait CollectVec: Iterator + Sized {
    fn collect_vec(self) -> Vec<Self::Item> {
        self.collect()
    }
}
impl<T> CollectVec for T where T: Iterator {}

#[cfg(test)]
mod tests {
    use super::CollectVec;

    #[test]
    fn collect_test() {
        let iter = 0..10;
        let data = iter.collect_vec();
        assert_eq!(data, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
