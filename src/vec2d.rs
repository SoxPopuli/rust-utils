use std::ops::{Index, IndexMut};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Error {
    SliceTooLong { size: usize },
    IteratorTooLong { size: usize },
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}
impl std::error::Error for Error {}

#[derive(Debug, Default, Clone)]
pub struct Vec2d<T> {
    data: Vec<T>,

    width: usize,
    height: usize,
}

impl<T> Vec2d<T> {
    fn with_capacity(width: usize, height: usize) -> Self {
        let buffer = Vec::with_capacity(Self::calc_size(width, height));
        Self {
            data: buffer,
            width,
            height,
        }
    }

    fn calc_size(width: usize, height: usize) -> usize {
        width * height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns total size. i.e. `width * height`
    pub fn size(&self) -> usize {
        self.width * self.height
    }

    /// Returns number of items in underlying buffer
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        (y * self.width) + x
    }

    /// Same as `get_index` but asserts x and y are in bounds
    fn get_index_assert(&self, x: usize, y: usize) -> usize {
        assert!(x < self.width());
        assert!(y < self.height());

        self.get_index(x, y)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        let idx = self.get_index(x, y);
        self.data.get_mut(idx)
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        let idx = self.get_index(x, y);
        self.data.get(idx)
    }

    /// Analogous to `Vec::insert`
    ///
    /// ---
    /// Inserts an element at position `index` within the vector, shifting all
    /// elements after it to the right.
    ///
    /// # Panics
    ///
    /// Panics if `index > len`.
    pub fn insert(&mut self, x: usize, y: usize, val: T) {
        let idx = self.get_index_assert(x, y);
        self.data.insert(idx, val)
    }

    pub fn from_iter<I: IntoIterator<Item = T>>(
        width: usize,
        height: usize,
        iter: I,
    ) -> Result<Self, Error> {
        let mut this = Self::with_capacity(width, height);
        let size = this.size();

        for (i, item) in iter.into_iter().enumerate() {
            if i >= size {
                return Err(Error::IteratorTooLong { size });
            }

            this.data.push(item)
        }

        Ok(this)
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.data.iter_mut()
    }

    pub fn resize(&mut self, new_width: usize, new_height: usize) {
        let new_size = Self::calc_size(new_width, new_height);

        if self.data.len() > new_size {
            self.data.truncate(new_size);
        }

        self.width = new_width;
        self.height = new_height;
    }

    /// Add item to vec
    /// Will add a new row if full
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` *bytes*.
    pub fn push(&mut self, item: T) {
        if self.len() >= self.size() {
            // Add new row
            self.height += 1;
        }

        self.data.push(item);
    }

    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![],
        }
    }
}

impl<T> Vec2d<T>
where
    T: Default,
{
    /// Take elements from slice
    /// Replaces elements with `T::default()`
    pub fn take_from(width: usize, height: usize, slice: &mut [T]) -> Result<Self, Error> {
        let size = Self::calc_size(width, height);
        let mut this = Self::with_capacity(width, height);

        if slice.len() > size {
            return Err(Error::SliceTooLong { size });
        }

        for item in slice.iter_mut() {
            this.data.push(std::mem::take(item));
        }

        Ok(this)
    }

    /// Insert element at index
    /// Expands vector if too small
    pub fn insert_at(&mut self, x: usize, y: usize, val: T) {
        let idx = self.get_index(x, y);
        while self.len() < idx {
            self.push(T::default());
        }

        self.push(val)
    }
}

impl<T> Vec2d<T>
where
    T: Clone,
{
    pub fn clone_from_slice(width: usize, height: usize, slice: &[T]) -> Self {
        let size = Self::calc_size(width, height);
        let data = slice.iter().take(size).cloned().collect();

        Self {
            data,
            width,
            height,
        }
    }
}

impl<T> Vec2d<T>
where
    T: Copy,
{
    pub fn new_with(width: usize, height: usize, val: T) -> Self {
        let data = vec![val; Self::calc_size(width, height)];
        Self {
            data,
            width,
            height,
        }
    }
}

impl<T> PartialEq for Vec2d<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        let same_elems = self.data == other.data;
        let same_width = self.width() == other.width();
        let same_height = self.height() == other.height();

        same_elems && same_width && same_height
    }
}

impl<T> From<Vec2d<T>> for Vec<T> {
    fn from(val: Vec2d<T>) -> Self {
        val.data
    }
}

impl<T> Index<usize> for Vec2d<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> Index<(usize, usize)> for Vec2d<T> {
    type Output = T;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        let index = self.get_index_assert(x, y);
        &self.data[index]
    }
}

impl<T> IndexMut<(usize, usize)> for Vec2d<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        let index = self.get_index_assert(x, y);
        &mut self.data[index]
    }
}

impl<T> IndexMut<usize> for Vec2d<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T> IntoIterator for Vec2d<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type AnyResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn with_capacity_test() {
        let width = 2;
        let height = 4;
        let v: Vec2d<i32> = Vec2d::with_capacity(width, height);

        assert_eq!(v.width, 2);
        assert_eq!(v.height, 4);
        assert_eq!(v.size(), 8);
    }

    #[test]
    fn from_iter_test() {
        let width = 2;
        let height = 4;

        let seq = 1..=8;

        let v = Vec2d::from_iter(width, height, seq).unwrap();

        assert_eq!(v.width, 2);
        assert_eq!(v.height, 4);
        assert_eq!(v.size(), 8);

        assert_eq!(v.data, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn from_iter_too_long_test() {
        let width = 2;
        let height = 4;

        let seq = 1..=9;

        let res = Vec2d::from_iter(width, height, seq);

        assert_eq!(res, Err(Error::IteratorTooLong { size: 8 }))
    }

    #[test]
    fn index_test() -> AnyResult<()> {
        let width = 2;
        let height = 4;

        #[rustfmt::skip]
        let seq = [
            1, 2,
            3, 4,
            5, 6,
            7, 8,
        ];

        let v = Vec2d::from_iter(width, height, seq)?;

        assert_eq!(v[1], 2);
        assert_eq!(v[(0, 0)], 1);
        assert_eq!(v[(1, 2)], 6);
        assert_eq!(v[(1, 3)], 8);

        Ok(())
    }

    #[test]
    fn push_test() {
        let width = 1;
        let height = 1;

        let mut v = Vec2d::from_iter(width, height, [1]).unwrap();

        v.push(2);
        assert_eq!(v.data, vec![1, 2]);
        assert_eq!(v.height, 2);

        v.push(3);
        assert_eq!(v.data, vec![1, 2, 3]);
        assert_eq!(v.height, 3);
    }

    #[test]
    fn get_index_test() {
        use std::panic::catch_unwind;

        let v: Vec2d<i32> = Vec2d {
            width: 2,
            height: 3,
            data: vec![],
        };

        assert_eq!(v.get_index_assert(0, 0), 0);
        assert_eq!(v.get_index_assert(1, 0), 1);
        assert_eq!(v.get_index_assert(0, 1), 2);
        assert_eq!(v.get_index_assert(1, 1), 3);
        assert_eq!(v.get_index_assert(0, 2), 4);
        assert_eq!(v.get_index_assert(1, 2), 5);

        catch_unwind(|| v.get_index_assert(2, 2)).expect_err("Should trip assert");
        catch_unwind(|| v.get_index_assert(0, 3)).expect_err("Should trip assert");
    }

    #[test]
    fn insert_at_test() {
        let width = 2;
        let height = 2;

        #[rustfmt::skip]
        let seq = [
            1, 2,
            3, 4,
        ];

        let mut v = Vec2d::from_iter(width, height, seq).unwrap();

        v.insert_at(1, 2, 6);

        assert_eq!(v.data, [1, 2, 3, 4, 0, 6]);
    }
}
