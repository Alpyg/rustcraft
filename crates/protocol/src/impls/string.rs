use std::fmt::Write;

use anyhow::ensure;
use bytes::BytesMut;

use crate::{Bounded, Decode, Encode, VarInt};

pub const MAX_STRING_LEN: usize = 32767;
//pub const MAX_TEXT_LEN: usize = 262144;

impl Encode for str {
    fn encode(&self, wtr: &mut BytesMut) -> anyhow::Result<()> {
        Bounded::<_, MAX_STRING_LEN>(self).encode(wtr)
    }
}

impl<'a> Decode<'a> for &'a str {
    fn decode(rdr: &mut &'a [u8]) -> anyhow::Result<Self> {
        Ok(Bounded::<_, MAX_STRING_LEN>::decode(rdr)?.0)
    }
}

impl<const MAX: usize> Encode for Bounded<&'_ str, MAX> {
    fn encode(&self, wtr: &mut BytesMut) -> anyhow::Result<()> {
        let len = self.encode_utf16().count();

        ensure!(
            len <= MAX,
            "string len exceeds maximum (expected <= {MAX}, got {len})"
        );

        VarInt(len as i32).encode(wtr)?;
        wtr.write_str(self)?;

        Ok(())
    }
}

impl<'a, const MAX: usize> Decode<'a> for Bounded<&'a str, MAX> {
    fn decode(rdr: &mut &'a [u8]) -> anyhow::Result<Self> {
        let len = VarInt::decode(rdr)?.0;
        ensure!(len >= 0, "attempt to decode string with negative length");
        let len = len as usize;
        ensure!(
            len <= rdr.len(),
            "not enough data remaining ({} bytes) to decode string of {len} bytes",
            rdr.len()
        );

        let (res, rest) = rdr.split_at(len);
        let res = std::str::from_utf8(res)?;

        let char_count = res.encode_utf16().count();
        ensure!(
            char_count <= MAX,
            "char count of string exceeds maximum (expected <= {MAX}, got {char_count})"
        );

        *rdr = rest;

        Ok(Bounded(res))
    }
}

impl Encode for String {
    fn encode(&self, wtr: &mut BytesMut) -> anyhow::Result<()> {
        self.as_str().encode(wtr)
    }
}

impl Decode<'_> for String {
    fn decode(rdr: &mut &[u8]) -> anyhow::Result<Self> {
        Ok(<&str>::decode(rdr)?.into())
    }
}

impl<const MAX: usize> Encode for Bounded<String, MAX> {
    fn encode(&self, wtr: &mut BytesMut) -> anyhow::Result<()> {
        Bounded::<_, MAX>(self.as_str()).encode(wtr)
    }
}

impl<const MAX: usize> Decode<'_> for Bounded<String, MAX> {
    fn decode(rdr: &mut &'_ [u8]) -> anyhow::Result<Self> {
        Ok(Bounded(Bounded::<&str, MAX>::decode(rdr)?.0.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string() {
        let a = "str";
        let b = "string".to_owned();
        let c = Bounded::<&str, 11>("bounded_str");
        let d = Bounded::<String, 14>("bounded_string".to_owned());
        let mut buf = BytesMut::new();

        let _ = a.encode(&mut buf);
        let _ = b.encode(&mut buf);
        let _ = c.encode(&mut buf);
        let _ = d.encode(&mut buf);

        let mut buf = &buf.freeze()[..];

        assert_eq!("str", <&str>::decode(&mut buf).unwrap());
        assert_eq!("string", String::decode(&mut buf).unwrap());
        assert_eq!(
            Bounded::<&str, 11>("bounded_str"),
            Bounded::<&str, 11>::decode(&mut buf).unwrap()
        );
        assert_eq!(
            Bounded::<String, 14>("bounded_string".to_owned()),
            Bounded::<String, 14>::decode(&mut buf).unwrap()
        );
    }
}
