use std::marker::PhantomData;

use uuid::Uuid;

use crate::{Decode, Encode};

impl Encode for Uuid {
    fn encode(&self, wtr: &mut bytes::BytesMut) -> anyhow::Result<()> {
        self.as_u128().encode(wtr)
    }
}

impl<'a> Decode<'a> for Uuid {
    fn decode(rdr: &mut &'a [u8]) -> anyhow::Result<Self> {
        u128::decode(rdr).map(Uuid::from_u128)
    }
}

impl<'a, T> Encode for PhantomData<&'a T> {
    fn encode(&self, _wtr: &mut bytes::BytesMut) -> anyhow::Result<()> {
        Ok(())
    }
}

impl<'a, T> Decode<'a> for PhantomData<&'a T> {
    fn decode(_rdr: &mut &'a [u8]) -> anyhow::Result<Self> {
        Ok(PhantomData {})
    }
}
