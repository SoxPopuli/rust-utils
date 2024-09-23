use std::io::Read;
use std::mem::size_of;

macro_rules! impl_from_bytes {
    ($t: ty) => {
        impl FromBytes for $t {
            type Error = std::io::Error;
            fn from_bytes_ne(mut data: impl Read) -> Result<Self, Self::Error> {
                let mut buf = [0u8; size_of::<Self>()];
                data.read_exact(&mut buf)?;
                Ok(<Self>::from_ne_bytes(buf))
            }
            fn from_bytes_le(mut data: impl Read) -> Result<Self, Self::Error> {
                let mut buf = [0u8; size_of::<Self>()];
                data.read_exact(&mut buf)?;
                Ok(<Self>::from_le_bytes(buf))
            }
            fn from_bytes_be(mut data: impl Read) -> Result<Self, Self::Error> {
                let mut buf = [0u8; size_of::<Self>()];
                data.read_exact(&mut buf)?;
                Ok(<Self>::from_be_bytes(buf))
            }
        }
    };

    ($($t: ty),+) => {
        $(impl_from_bytes!($t);)+
    }
}

pub trait FromBytes: Sized {
    type Error;
    fn from_bytes_ne(data: impl Read) -> Result<Self, Self::Error>;
    fn from_bytes_le(data: impl Read) -> Result<Self, Self::Error>;
    fn from_bytes_be(data: impl Read) -> Result<Self, Self::Error>;
}

impl_from_bytes!(i8, i16, i32, i64, i128);
impl_from_bytes!(u8, u16, u32, u64, u128);
impl_from_bytes!(f32, f64);

impl FromBytes for bool {
    type Error = std::io::Error;

    fn from_bytes_ne(mut data: impl Read) -> Result<Self, Self::Error> {
        let mut buf = [0u8; 1];
        data.read_exact(&mut buf)?;
        match buf[0] {
            0 => Ok(false),
            _ => Ok(true),
        }
    }
    fn from_bytes_le(data: impl Read) -> Result<Self, Self::Error> {
        Self::from_bytes_ne(data)
    }
    fn from_bytes_be(data: impl Read) -> Result<Self, Self::Error> {
        Self::from_bytes_ne(data)
    }
}

pub fn from_bytes_ne<T: FromBytes>(data: impl Read) -> Result<T, T::Error> {
    T::from_bytes_ne(data)
}

pub fn from_bytes_le<T: FromBytes>(data: impl Read) -> Result<T, T::Error> {
    T::from_bytes_le(data)
}

pub fn from_bytes_be<T: FromBytes>(data: impl Read) -> Result<T, T::Error> {
    T::from_bytes_be(data)
}

#[cfg(test)]
mod tests {
    use crate::byte_readers::from_bytes_le;

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
        data.rewind().unwrap();

        let x: i32 = from_bytes_le(&mut data).unwrap();
        assert_eq!(x, 1);
    }
}
