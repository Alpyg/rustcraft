use anyhow::anyhow;
use bytes::{Buf, BufMut, Bytes, BytesMut};

use crate::protocol::{Decode, Encode};

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
define_primitive!(f32, get_f32, put_f32);
define_primitive!(f64, get_f64, put_f64);

macro_rules! define_varnum {
    ($name:ident, $type:ty, $container_type:ty, $max_size:literal) => {
        #[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Hash, Ord, Eq)]

        pub struct $name(pub $type);

        impl Encode for $name {
            fn encode(&self, buf: &mut BytesMut) {
                let mut val = self.0 as $container_type;

                loop {
                    #[allow(overflowing_literals)]
                    if val & (0xFFFFFFFFFFFFFF80 as $container_type) == 0 {
                        buf.put_u8(val as u8);
                        return;
                    }

                    buf.put_u8(val as u8 | 0x80);
                    val >>= 7;
                }
            }
        }

        impl Decode for $name {
            fn decode(buf: &mut Bytes) -> anyhow::Result<Self> {
                let mut val = 0 as $container_type;
                for i in 0..$max_size {
                    let byte = buf.get_u8();
                    val |= (byte as $container_type & 0x7F) << (i * 7);

                    if byte & 0x80 == 0 {
                        return Ok($name(val as $type));
                    }
                }

                Err(anyhow!("VarNum too large."))
            }
        }
    };
}

define_varnum!(VarInt, i32, u32, 5);
define_varnum!(VarLong, i64, u64, 10);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitive_encode_decode() {
        let a: i32 = 0x11223344;
        let b: u64 = 0x1122334455667788;
        let mut buf = BytesMut::new();

        a.encode(&mut buf);
        b.encode(&mut buf);

        assert_eq!(0x1122, u16::decode(&mut buf).unwrap());
        assert_eq!(0x33441122, u32::decode(&mut buf).unwrap());
    }

    #[test]
    fn varnum_encode_decode() {
        let mut buf = BytesMut::new();

        VarInt(0).encode(&mut buf);
        VarInt(1).encode(&mut buf);
        VarInt(2).encode(&mut buf);
        VarInt(127).encode(&mut buf);
        VarInt(128).encode(&mut buf);
        VarInt(255).encode(&mut buf);
        VarInt(25565).encode(&mut buf);
        VarInt(2147483647).encode(&mut buf);

        VarLong(9223372036854775807).encode(&mut buf);
        VarLong(-1).encode(&mut buf);
        VarLong(-2147483648).encode(&mut buf);
        VarLong(-9223372036854775808).encode(&mut buf);

        assert_eq!(0, VarInt::decode(&mut buf).unwrap().0);
        assert_eq!(1, VarInt::decode(&mut buf).unwrap().0);
        assert_eq!(2, VarInt::decode(&mut buf).unwrap().0);
        assert_eq!(127, VarInt::decode(&mut buf).unwrap().0);
        assert_eq!(128, VarInt::decode(&mut buf).unwrap().0);
        assert_eq!(255, VarInt::decode(&mut buf).unwrap().0);
        assert_eq!(25565, VarInt::decode(&mut buf).unwrap().0);
        assert_eq!(2147483647, VarInt::decode(&mut buf).unwrap().0);

        assert_eq!(9223372036854775807, VarLong::decode(&mut buf).unwrap().0);
        assert_eq!(-1, VarLong::decode(&mut buf).unwrap().0);
        assert_eq!(-2147483648, VarLong::decode(&mut buf).unwrap().0);
        assert_eq!(-9223372036854775808, VarLong::decode(&mut buf).unwrap().0);
    }
}
