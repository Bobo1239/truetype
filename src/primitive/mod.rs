//! Primitive data types.

use std::mem;

use Result;
use band::{Band, Value};

pub type Byte = u8;
pub type Char = i8;

pub type UShort = u16;
pub type Short = i16;

pub type UFWord = u16;
pub type FWord = i16;

pub type ULong = u32;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Fixed(pub u32);

pub type LongDateTime = i64;

impl Fixed {
    pub fn as_f32(&self) -> f32 {
        const SCALE: f32 = 1f32 / (1 << 16) as f32;
        SCALE * (self.0 as f32)
    }
}

macro_rules! fill(
    ($band:ident, $count:expr, $buffer:ident) => (
        if try!(::std::io::Read::read($band, &mut $buffer)) != $count {
            return raise!("failed to read as much as needed");
        }
    );
);

macro_rules! read(
    ($band:ident, $size:expr) => (unsafe {
        let mut buffer: [u8; $size] = mem::uninitialized();
        fill!($band, $size, buffer);
        mem::transmute(buffer)
    });
);

macro_rules! implement {
    ($name:ident, 1) => (impl Value for $name {
        fn read<T: Band>(band: &mut T) -> Result<Self> {
            Ok(read!(band, 1))
        }
    });
    ($name:ident, $size:expr) => (impl Value for $name {
        fn read<T: Band>(band: &mut T) -> Result<Self> {
            Ok($name::from_be(read!(band, $size)))
        }
    });
}

implement!(i8, 1);
implement!(u8, 1);

implement!(i16, 2);
implement!(u16, 2);

implement!(u32, 4);

implement!(i64, 8);

impl Value for Fixed {
    #[inline(always)]
    fn read<T: Band>(band: &mut T) -> Result<Self> {
        Ok(Fixed(try!(Value::read(band))))
    }
}
