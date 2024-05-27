use anyhow::ensure;
use bytes::BytesMut;
use derive_more::{AsRef, Deref, DerefMut, From};

use crate::{Decode, Encode, VarInt};

impl<T: Encode> Encode for Option<T> {
    fn encode(&self, wtr: &mut BytesMut) -> anyhow::Result<()> {
        match self {
            Some(t) => {
                true.encode(wtr)?;
                t.encode(wtr)
            }
            None => false.encode(wtr),
        }
    }
}

impl<'a, T: Decode<'a>> Decode<'a> for Option<T> {
    fn decode(rdr: &mut &'a [u8]) -> anyhow::Result<Self> {
        Ok(match bool::decode(rdr)? {
            true => Some(T::decode(rdr)?),
            false => None,
        })
    }
}

impl<T: Encode, const N: usize> Encode for [T; N] {
    fn encode(&self, wtr: &mut BytesMut) -> anyhow::Result<()> {
        T::encode_slice(self, wtr)
    }
}

impl<'a, T: Decode<'a>, const N: usize> Decode<'a> for [T; N] {
    fn decode(rdr: &mut &'a [u8]) -> anyhow::Result<Self> {
        std::array::try_from_fn(|_i| T::decode(rdr))
    }
}

impl<T: Encode> Encode for [T] {
    fn encode(&self, wtr: &mut BytesMut) -> anyhow::Result<()> {
        let len = self.len();
        ensure!(
            len <= i32::MAX as usize,
            "length of {} slice exceeds i32::MAX (got {len})",
            std::any::type_name::<T>()
        );

        VarInt(len as i32).encode(wtr)?;
        T::encode_slice(self, wtr)
    }
}

impl<T: Encode> Encode for Vec<T> {
    fn encode(&self, wtr: &mut BytesMut) -> anyhow::Result<()> {
        self.as_slice().encode(wtr)
    }
}

impl<'a, T: Decode<'a>> Decode<'a> for Vec<T> {
    fn decode(rdr: &mut &'a [u8]) -> anyhow::Result<Self> {
        let len = VarInt::decode(rdr)?.0;
        ensure!(len >= 0, "attempt to decode Vec with negative length");
        let len = len as usize;

        let mut vec = Vec::<T>::with_capacity(len);

        for _ in 0..len {
            vec.push(T::decode(rdr)?);
        }

        Ok(vec)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, DerefMut, AsRef, From)]
pub struct LenPrefixed<T>(pub Vec<T>);

impl<T: Encode> Encode for LenPrefixed<T> {
    fn encode(&self, wtr: &mut bytes::BytesMut) -> anyhow::Result<()> {
        VarInt(self.len() as i32).encode(wtr)?;

        for i in self.iter() {
            i.encode(wtr)?
        }

        Ok(())
    }
}

impl<'a, T: Decode<'a>> Decode<'a> for LenPrefixed<T> {
    fn decode(rdr: &mut &'a [u8]) -> anyhow::Result<Self> {
        let len = VarInt::decode(rdr)?.0 as usize;

        let mut vec = Vec::<T>::with_capacity(len);
        for _ in 0..len {
            vec.push(Decode::decode(rdr)?)
        }

        Ok(Self(vec))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn container_encode_decode() {
        let a = Some("aaa".to_string());
        let b: Option<u128> = None;
        let c = vec![0; 10];
        let d: [u16; 3] = [7, 7, 7];
        let mut buf = BytesMut::new();

        let _ = a.encode(&mut buf);

        assert_eq!(5, buf.len());

        let _ = b.encode(&mut buf);
        let _ = c.encode(&mut buf);
        let _ = d.encode(&mut buf);

        let mut buf = &buf.freeze()[..];

        assert_eq!(
            Some("aaa".to_string()),
            <Option<String>>::decode(&mut buf).unwrap()
        );
        assert_eq!(None, <Option<u128>>::decode(&mut buf).unwrap());
        assert_eq!(vec![0; 10], <Vec<i32>>::decode(&mut buf).unwrap());
        assert_eq!([7, 7, 7], <[u16; 3]>::decode(&mut buf).unwrap());
    }
}
