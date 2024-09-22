use std::io::Read;
use std::mem::size_of;

macro_rules! impl_from_bytes {
    ($t: ty) => {
        impl FromBytes for $t {
            type Output = $t;
            type Error = std::io::Error;
            fn from_bytes_ne(mut data: impl Read) -> Result<Self::Output, Self::Error> {
                let mut buf = [0u8; size_of::<$t>()];
                data.read_exact(&mut buf)?;
                Ok(<$t>::from_ne_bytes(buf))
            }
            fn from_bytes_le(mut data: impl Read) -> Result<Self::Output, Self::Error> {
                let mut buf = [0u8; size_of::<$t>()];
                data.read_exact(&mut buf)?;
                Ok(<$t>::from_le_bytes(buf))
            }
            fn from_bytes_be(mut data: impl Read) -> Result<Self::Output, Self::Error> {
                let mut buf = [0u8; size_of::<$t>()];
                data.read_exact(&mut buf)?;
                Ok(<$t>::from_be_bytes(buf))
            }
        }
    };
}

pub trait FromBytes {
    type Output;
    type Error;
    fn from_bytes_ne(data: impl Read) -> Result<Self::Output, Self::Error>;
    fn from_bytes_le(data: impl Read) -> Result<Self::Output, Self::Error>;
    fn from_bytes_be(data: impl Read) -> Result<Self::Output, Self::Error>;
}
impl_from_bytes!(i8);
impl_from_bytes!(i16);
impl_from_bytes!(i32);
impl_from_bytes!(i64);
impl_from_bytes!(i128);

impl_from_bytes!(u8);
impl_from_bytes!(u16);
impl_from_bytes!(u32);
impl_from_bytes!(u64);
impl_from_bytes!(u128);

impl_from_bytes!(f32);
impl_from_bytes!(f64);

impl FromBytes for bool {
    type Output = bool;
    type Error = std::io::Error;

    fn from_bytes_ne(mut data: impl Read) -> Result<Self::Output, Self::Error> {
        let mut buf = [0u8; 1];
        data.read_exact(&mut buf)?;
        match buf[0] {
            0 => Ok(false),
            _ => Ok(true),
        }
    }
    fn from_bytes_le(data: impl Read) -> Result<Self::Output, Self::Error> {
        Self::from_bytes_ne(data)
    }
    fn from_bytes_be(data: impl Read) -> Result<Self::Output, Self::Error> {
        Self::from_bytes_ne(data)
    }
}

#[cfg(test)]
mod tests {
    use super::FromBytes;
    use std::io::{Cursor, Seek};

    #[test]
    fn read_bytes_test() {
        let mut data = Cursor::new([1, 0, 0, 0]);
        let x = i32::from_bytes_le(&mut data).unwrap();
        assert_eq!(x, 1);
        data.rewind().unwrap();

        let x = i32::from_bytes_be(&mut data).unwrap();
        assert_eq!(x, 16777216);
    }
}
