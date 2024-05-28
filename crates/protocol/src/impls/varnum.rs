use anyhow::anyhow;
use byteorder::ReadBytesExt;
use bytes::{BufMut, BytesMut};
use derive_more::{Deref, DerefMut, From, Into};
use mem_macros::size_of;

use crate::{Decode, Encode};

macro_rules! define_varnum {
    ($name:ident, $type:ty, $container_type:ty, $max_size:literal) => {
        #[derive(
            Debug, Copy, Clone, PartialOrd, PartialEq, Hash, Ord, Eq, Deref, DerefMut, From, Into,
        )]
        pub struct $name(pub $type);

        impl $name {
            pub fn size(self) -> usize {
                match self.0 {
                    0 => 1,
                    n => (size_of!($container_type) * 8 - 1 - n.leading_zeros() as usize) / 7 + 1,
                }
            }
        }

        impl Encode for $name {
            fn encode(&self, buf: &mut BytesMut) -> anyhow::Result<()> {
                let mut val = self.0 as $container_type;

                loop {
                    #[allow(overflowing_literals)]
                    if val & (0xFFFFFFFFFFFFFF80 as $container_type) == 0 {
                        buf.put_u8(val as u8);
                        return Ok(());
                    }

                    buf.put_u8(val as u8 | 0x80);
                    val >>= 7;
                }
            }
        }

        impl Decode<'_> for $name {
            fn decode(buf: &mut &[u8]) -> anyhow::Result<Self> {
                let mut val = 0 as $container_type;
                for i in 0..$max_size {
                    let byte = buf.read_u8()?;
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

        let _ = a.encode(&mut buf);
        let _ = b.encode(&mut buf);

        let mut buf = &buf.freeze()[..];

        assert_eq!(0x1122, u16::decode(&mut buf).unwrap());
        assert_eq!(0x33441122, u32::decode(&mut buf).unwrap());
    }

    #[test]
    fn varnum_encode_decode() {
        let mut buf = BytesMut::new();

        let _ = VarInt(0).encode(&mut buf);
        let _ = VarInt(1).encode(&mut buf);
        let _ = VarInt(2).encode(&mut buf);
        let _ = VarInt(127).encode(&mut buf);
        let _ = VarInt(128).encode(&mut buf);
        let _ = VarInt(255).encode(&mut buf);
        let _ = VarInt(25565).encode(&mut buf);
        let _ = VarInt(2147483647).encode(&mut buf);

        let _ = VarLong(9223372036854775807).encode(&mut buf);
        let _ = VarLong(-1).encode(&mut buf);
        let _ = VarLong(-2147483648).encode(&mut buf);
        let _ = VarLong(-9223372036854775808).encode(&mut buf);

        let mut buf = &buf.freeze()[..];

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
