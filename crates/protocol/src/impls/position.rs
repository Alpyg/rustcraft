use bytes::{Buf, BufMut, BytesMut};
use derive_more::From;

use crate::{Decode, Encode};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Encode for Position {
    fn encode(&self, wtr: &mut BytesMut) -> anyhow::Result<()> {
        wtr.put_u64(
            ((self.x as u64 & 0x3FFFFFF) << 38)
                | ((self.z as u64 & 0x3FFFFFF) << 12)
                | (self.y as u64 & 0xFFF),
        );
        Ok(())
    }
}

impl<'a> Decode<'a> for Position {
    fn decode(rdr: &mut &'a [u8]) -> anyhow::Result<Self> {
        let val = rdr.get_u64();
        Ok(Self {
            x: (val >> 38) as i32,
            y: (val << 52 >> 52) as i32,
            z: (val << 26 >> 38) as i32,
        })
    }
}
