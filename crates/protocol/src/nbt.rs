use bytes::{Buf, BufMut, Bytes, BytesMut};
use derive_more::{Deref, DerefMut, From, TryInto};
use indexmap::IndexMap;

use crate::{Decode, Encode};

#[derive(Debug, Clone, PartialEq, From, TryInto)]
#[try_into(owned, ref, ref_mut)]
#[repr(u8)]
pub enum NBT {
    End(),
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Bytes),
    String(String),
    List(List),
    Compound(Compound),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl NBT {
    fn id(&self) -> u8 {
        match self {
            NBT::End() => 0,
            NBT::Byte(_) => 1,
            NBT::Short(_) => 2,
            NBT::Int(_) => 3,
            NBT::Long(_) => 4,
            NBT::Float(_) => 5,
            NBT::Double(_) => 6,
            NBT::ByteArray(_) => 7,
            NBT::String(_) => 8,
            NBT::List(_) => 9,
            NBT::Compound(_) => 10,
            NBT::IntArray(_) => 11,
            NBT::LongArray(_) => 12,
        }
    }

    fn encode(&self, wtr: &mut BytesMut) -> anyhow::Result<()> {
        let id = self.id();
        match self {
            NBT::End() => wtr.put_u8(0),
            NBT::Byte(val) => wtr.put_i8(*val),
            NBT::Short(val) => wtr.put_i16(*val),
            NBT::Int(val) => wtr.put_i32(*val),
            NBT::Long(val) => wtr.put_i64(*val),
            NBT::Float(val) => wtr.put_f32(*val),
            NBT::Double(val) => wtr.put_f64(*val),
            NBT::ByteArray(val) => {
                wtr.put_i32(val.len() as i32);
                wtr.put_slice(val);
            }
            NBT::String(val) => {
                wtr.put_u16(val.len() as u16);
                wtr.put_slice(val.as_bytes());
            }
            NBT::List(val) => {
                id.encode(wtr)?;
                if val.is_empty() {
                    wtr.put_u8(0);
                    wtr.put_i32(0);
                } else {
                    let tag_id = &val.first().unwrap().id();
                    wtr.put_u8(*tag_id);
                    wtr.put_i32(val.len() as i32);
                    for tag in &val.0 {
                        tag.encode(wtr)?;
                    }
                }
            }
            NBT::Compound(val) => {
                id.encode(wtr)?;
                for (key, tag) in &val.0 {
                    let tag_id = tag.id();
                    tag_id.encode(wtr)?;
                    if !tag.id() == 0 {
                        wtr.put_u16(key.len() as u16);
                        wtr.put_slice(key.as_bytes());
                    }
                    tag.encode(wtr)?;
                }
            }
            NBT::IntArray(val) => {
                wtr.put_i32(val.len() as i32);
                for val in val {
                    wtr.put_i32(*val);
                }
            }
            NBT::LongArray(val) => {
                wtr.put_i32(val.len() as i32);
                for val in val {
                    wtr.put_i64(*val);
                }
            }
        }

        Ok(())
    }

    fn decode(rdr: &mut &[u8], id: u8) -> anyhow::Result<Self> {
        match id {
            0 => Ok(NBT::End()),
            1 => {
                let val = rdr.get_i8();
                Ok(NBT::Byte(val))
            }
            2 => {
                let val = rdr.get_i16();
                Ok(NBT::Short(val))
            }
            3 => {
                let val = rdr.get_i32();
                Ok(NBT::Int(val))
            }
            4 => {
                let val = rdr.get_i64();
                Ok(NBT::Long(val))
            }
            5 => {
                let val = rdr.get_f32();
                Ok(NBT::Float(val))
            }
            6 => {
                let val = rdr.get_f64();
                Ok(NBT::Double(val))
            }
            7 => {
                let len = rdr.get_i32() as usize;
                let mut buf = vec![0; len];
                rdr.copy_to_slice(&mut buf);
                Ok(NBT::ByteArray(buf.into()))
            }
            8 => {
                let len = rdr.get_u16() as usize;
                let mut buf = vec![0; len];
                rdr.copy_to_slice(&mut buf);
                let val = String::from_utf8(buf)?;
                Ok(NBT::String(val))
            }
            9 => {
                let id = rdr.get_u8();
                let len = rdr.get_i32();

                let mut list = List::with_capacity(len as usize);
                if len <= 0 {
                    Ok(NBT::List(List::new()))
                } else {
                    for _ in 0..len {
                        let tag = NBT::decode(rdr, id)?;
                        list.push(tag);
                    }
                    Ok(NBT::List(list))
                }
            }
            10 => {
                let mut map = Compound::new();
                while !rdr.is_empty() {
                    let id = rdr.get_u8();
                    if id == 0 {
                        break;
                    }

                    let name_len = rdr.get_u16() as usize;
                    let mut buf = vec![0; name_len];
                    rdr.copy_to_slice(&mut buf);
                    let name = String::from_utf8(buf)?;

                    let tag = NBT::decode(rdr, id)?;
                    map.insert(name, tag);
                }
                Ok(NBT::Compound(map))
            }
            11 => {
                let len = rdr.get_i32() as usize;
                let mut vec = Vec::with_capacity(len);
                for _ in 0..len {
                    vec.push(rdr.get_i32());
                }
                Ok(NBT::IntArray(vec))
            }
            12 => {
                let len = rdr.get_i32() as usize;
                let mut vec = Vec::with_capacity(len);
                for _ in 0..len {
                    vec.push(rdr.get_i64());
                }
                Ok(NBT::LongArray(vec))
            }
            _ => Err(anyhow::anyhow!("Unknown tag ID: {}", id)),
        }
    }
}

#[derive(Debug, Clone, Deref, DerefMut, PartialEq)]
pub struct List(Vec<NBT>);

impl List {
    fn new() -> Self {
        Self(Vec::<NBT>::new())
    }

    fn with_capacity(capacity: usize) -> Self {
        Self(Vec::<NBT>::with_capacity(capacity))
    }
}

#[derive(Debug, Clone, Deref, DerefMut, PartialEq)]
pub struct Compound(IndexMap<String, NBT>);

impl Compound {
    fn new() -> Self {
        Self(IndexMap::<String, NBT>::new())
    }
}

impl Encode for NBT {
    fn encode(&self, wtr: &mut BytesMut) -> anyhow::Result<()> {
        self.encode(wtr)
    }
}

impl<'a> Decode<'a> for NBT {
    fn decode(rdr: &mut &'a [u8]) -> anyhow::Result<Self> {
        let id = u8::decode(rdr)?;
        NBT::decode(rdr, id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list() {
        let mut list = List::new();

        list.push(NBT::End());
        list.push(NBT::Byte(7));

        assert!(matches!(list.first().unwrap(), NBT::End()));
        assert!(matches!(list.get(1).unwrap(), NBT::Byte(7)));
    }

    #[test]
    fn compound() {
        let mut compound = Compound::new();

        compound.insert("test".to_owned(), NBT::End());

        assert!(compound.contains_key(&"test".to_string()));
        assert!(matches!(
            compound.get(&"test".to_string()).unwrap(),
            NBT::End()
        ));
    }

    #[test]
    fn test_test() {
        let mut data = vec![10];
        data.extend_from_slice(include_bytes!("testdata/test.nbt"));

        let tag: NBT = Decode::decode(&mut data.as_slice()).unwrap();

        if let NBT::Compound(mut compound) = tag {
            let hello: &mut Compound = compound
                .get_mut(&"hello world".to_owned())
                .unwrap()
                .try_into()
                .unwrap();

            assert_eq!(
                hello.get(&"name".to_owned()).unwrap(),
                &NBT::String("Bananrama".to_owned())
            );

            hello.insert("name".to_owned(), NBT::String("awawa".to_owned()));

            assert_eq!(
                hello.get(&"name".to_owned()).unwrap(),
                &NBT::String("awawa".to_owned())
            );
        }
    }

    #[test]
    fn bigtest_test() {
        use flate2::read::GzDecoder;
        use std::io::Read;

        let uncompressed = include_bytes!("testdata/bigtest.nbt");
        let mut data = vec![10];

        let mut gz = GzDecoder::new(uncompressed.as_slice());
        gz.read_to_end(&mut data).unwrap();

        let _tag: NBT = Decode::decode(&mut data.as_slice()).unwrap();

        //println!("{:#?}", _tag);
    }
}
