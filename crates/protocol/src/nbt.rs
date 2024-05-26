use anyhow::Ok;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use derive_more::{Deref, DerefMut};
use indexmap::IndexMap;

use crate::{Decode, Encode};

#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum Tag {
    End,
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

impl Tag {
    fn id(&self) -> u8 {
        match self {
            Tag::End => 0,
            Tag::Byte(_) => 1,
            Tag::Short(_) => 2,
            Tag::Int(_) => 3,
            Tag::Long(_) => 4,
            Tag::Float(_) => 5,
            Tag::Double(_) => 6,
            Tag::ByteArray(_) => 7,
            Tag::String(_) => 8,
            Tag::List(_) => 9,
            Tag::Compound(_) => 10,
            Tag::IntArray(_) => 11,
            Tag::LongArray(_) => 12,
        }
    }

    fn encode(&self, wtr: &mut BytesMut) -> anyhow::Result<()> {
        let id = self.id();
        match self {
            Tag::End => wtr.put_u8(0),
            Tag::Byte(val) => wtr.put_i8(*val),
            Tag::Short(val) => wtr.put_i16(*val),
            Tag::Int(val) => wtr.put_i32(*val),
            Tag::Long(val) => wtr.put_i64(*val),
            Tag::Float(val) => wtr.put_f32(*val),
            Tag::Double(val) => wtr.put_f64(*val),
            Tag::ByteArray(val) => {
                wtr.put_i32(val.len() as i32);
                wtr.put_slice(val);
            }
            Tag::String(val) => {
                wtr.put_u16(val.len() as u16);
                wtr.put_slice(val.as_bytes());
            }
            Tag::List(val) => {
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
            Tag::Compound(val) => {
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
            Tag::IntArray(val) => {
                wtr.put_i32(val.len() as i32);
                for val in val {
                    wtr.put_i32(*val);
                }
            }
            Tag::LongArray(val) => {
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
            0 => Ok(Tag::End),
            1 => {
                let val = rdr.get_i8();
                Ok(Tag::Byte(val))
            }
            2 => {
                let val = rdr.get_i16();
                Ok(Tag::Short(val))
            }
            3 => {
                let val = rdr.get_i32();
                Ok(Tag::Int(val))
            }
            4 => {
                let val = rdr.get_i64();
                Ok(Tag::Long(val))
            }
            5 => {
                let val = rdr.get_f32();
                Ok(Tag::Float(val))
            }
            6 => {
                let val = rdr.get_f64();
                Ok(Tag::Double(val))
            }
            7 => {
                let len = rdr.get_i32() as usize;
                let mut buf = vec![0; len];
                rdr.copy_to_slice(&mut buf);
                Ok(Tag::ByteArray(buf.into()))
            }
            8 => {
                let len = rdr.get_u16() as usize;
                let mut buf = vec![0; len];
                rdr.copy_to_slice(&mut buf);
                let val = String::from_utf8(buf)?;
                Ok(Tag::String(val))
            }
            9 => {
                let id = rdr.get_u8();
                let len = rdr.get_i32();

                let mut list = List::with_capacity(len as usize);
                if len <= 0 {
                    Ok(Tag::List(List::new()))
                } else {
                    for _ in 0..len {
                        let tag = Tag::decode(rdr, id)?;
                        list.push(tag);
                    }
                    Ok(Tag::List(list))
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

                    let tag = Tag::decode(rdr, id)?;
                    map.insert(name, tag);
                }
                Ok(Tag::Compound(map))
            }
            11 => {
                let len = rdr.get_i32() as usize;
                let mut vec = Vec::with_capacity(len);
                for _ in 0..len {
                    vec.push(rdr.get_i32());
                }
                Ok(Tag::IntArray(vec))
            }
            12 => {
                let len = rdr.get_i32() as usize;
                let mut vec = Vec::with_capacity(len);
                for _ in 0..len {
                    vec.push(rdr.get_i64());
                }
                Ok(Tag::LongArray(vec))
            }
            _ => Err(anyhow::anyhow!("Unknown tag ID: {}", id)),
        }
    }
}

#[derive(Debug, Clone, Deref, DerefMut, PartialEq)]
pub struct List(Vec<Tag>);

impl List {
    fn new() -> Self {
        Self(Vec::<Tag>::new())
    }

    fn with_capacity(capacity: usize) -> Self {
        Self(Vec::<Tag>::with_capacity(capacity))
    }
}

#[derive(Debug, Clone, Deref, DerefMut, PartialEq)]
pub struct Compound(IndexMap<String, Tag>);

impl Compound {
    fn new() -> Self {
        Self(IndexMap::<String, Tag>::new())
    }
}

impl Encode for Tag {
    fn encode(&self, wtr: &mut BytesMut) -> anyhow::Result<()> {
        self.encode(wtr)
    }
}

impl<'a> Decode<'a> for Tag {
    fn decode(rdr: &mut &'a [u8]) -> anyhow::Result<Self> {
        Tag::decode(rdr, 10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list() {
        let mut list = List::new();

        list.push(Tag::End);
        list.push(Tag::Byte(7));

        assert!(matches!(list.first().unwrap(), Tag::End));
        assert!(matches!(list.get(1).unwrap(), Tag::Byte(7)));
    }

    #[test]
    fn compound() {
        let mut compound = Compound::new();

        compound.insert("test".to_owned(), Tag::End);

        assert!(compound.contains_key(&"test".to_string()));
        assert!(matches!(
            compound.get(&"test".to_string()).unwrap(),
            Tag::End
        ));
    }

    #[test]
    fn test_test() {
        let data = include_bytes!("testdata/test.nbt");

        let _tag: Tag = Decode::decode(&mut data.as_slice()).unwrap();

        //if let Tag::Compound(comp) = tag {
        //    if let Tag::Compound(comp) = comp.first().unwrap().1 {
        //        println!("{:#?}", comp.first().unwrap());
        //    }
        //}
    }

    #[test]
    fn bigtest_test() {
        use flate2::read::GzDecoder;
        use std::io::Read;

        let uncompressed = include_bytes!("testdata/bigtest.nbt");
        let mut data = vec![];

        let mut gz = GzDecoder::new(uncompressed.as_slice());
        gz.read_to_end(&mut data).unwrap();

        println!("{} {}", uncompressed.len(), data.len());

        let _tag: Tag = Decode::decode(&mut data.as_slice()).unwrap();

        //println!("{:#?}", tag);
    }
}
