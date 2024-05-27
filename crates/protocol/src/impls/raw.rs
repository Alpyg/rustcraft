use std::{io::Write, mem};

use anyhow::ensure;
use bytes::{BufMut, BytesMut};
use derive_more::{AsRef, Deref, DerefMut, From};

use crate::{Bounded, Decode, Encode};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, DerefMut, AsRef, From)]
pub struct RawBytes<'a>(pub &'a [u8]);

impl Encode for RawBytes<'_> {
    fn encode(&self, wtr: &mut BytesMut) -> anyhow::Result<()> {
        Ok(wtr.writer().write_all(self)?)
    }
}

impl<'a> Decode<'a> for RawBytes<'a> {
    fn decode(rdr: &mut &'a [u8]) -> anyhow::Result<Self> {
        Ok(Self(mem::take(rdr)))
    }
}

impl<const MAX_BYTES: usize> Encode for Bounded<RawBytes<'_>, MAX_BYTES> {
    fn encode(&self, w: &mut BytesMut) -> anyhow::Result<()> {
        ensure!(
            self.len() <= MAX_BYTES,
            "cannot encode more than {MAX_BYTES} raw bytes (got {} bytes)",
            self.len()
        );

        self.0.encode(w)
    }
}

impl<'a, const MAX_BYTES: usize> Decode<'a> for Bounded<RawBytes<'a>, MAX_BYTES> {
    fn decode(r: &mut &'a [u8]) -> anyhow::Result<Self> {
        ensure!(
            r.len() <= MAX_BYTES,
            "remainder of input exceeds max of {MAX_BYTES} bytes (got {} bytes)",
            r.len()
        );

        Ok(Bounded(RawBytes::decode(r)?))
    }
}
