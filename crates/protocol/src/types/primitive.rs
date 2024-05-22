use bytes::{Buf, BufMut, Bytes, BytesMut};

use crate::{Decode, Encode};

impl Encode for bool {
    fn encode(&self, buf: &mut BytesMut) {
        buf.put_u8(*self as u8);
    }
}

impl Decode for bool {
    fn decode(buf: &mut Bytes) -> anyhow::Result<Self> {
        Ok(buf.get_u8() != 0)
    }
}

macro_rules! define_primitive {
    ($type:ty, $read:ident, $write:ident) => {
        impl Encode for $type {
            fn encode(&self, buf: &mut BytesMut) {
                buf.$write(*self)
            }
        }

        impl Decode for $type {
            fn decode(buf: &mut Bytes) -> anyhow::Result<Self> {
                Ok(buf.$read())
            }
        }
    };
}

define_primitive!(u8, get_u8, put_u8);
define_primitive!(i8, get_i8, put_i8);
define_primitive!(u16, get_u16, put_u16);
define_primitive!(i16, get_i16, put_i16);
define_primitive!(u32, get_u32, put_u32);
define_primitive!(i32, get_i32, put_i32);
define_primitive!(u64, get_u64, put_u64);
define_primitive!(i64, get_i64, put_i64);
define_primitive!(u128, get_u128, put_u128);
define_primitive!(i128, get_i128, put_i128);
define_primitive!(f32, get_f32, put_f32);
define_primitive!(f64, get_f64, put_f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitive_encode_decode() {
        let a = true;
        let b: i32 = 0x11223344;
        let c: u64 = 0x1122334455667788;
        let mut buf = BytesMut::new();

        a.encode(&mut buf);
        b.encode(&mut buf);
        c.encode(&mut buf);

        let mut buf: Bytes = buf.freeze();

        assert_eq!(0x01, u8::decode(&mut buf).unwrap());
        assert_eq!(0x1122, u16::decode(&mut buf).unwrap());
        assert_eq!(0x33441122, u32::decode(&mut buf).unwrap());
    }
}
