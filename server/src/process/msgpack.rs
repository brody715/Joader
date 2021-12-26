use rmp::decode::{read_bin_len, read_marker};
use rmp::encode::{write_array_len, write_bin, write_u16, write_u32, write_u8};
use rmp::Marker;
use std::collections::HashMap;
use std::convert::TryInto;
use std::io::{Cursor, Seek, SeekFrom};

// https://github.com/msgpack/msgpack/blob/master/spec.md
const U8_MAX: usize = u8::MAX as usize;
const U16_MAX: usize = u16::MAX as usize;
const U32_MAX: usize = u32::MAX as usize;

#[derive(Debug)]
pub enum MsgObject<'a> {
    Array(Vec<Box<MsgObject<'a>>>),
    Bin(&'a [u8]),
    Map(HashMap<&'a str, Box<MsgObject<'a>>>),
    UInt(&'a [u8]),
    Bool(bool),
}

#[inline]
fn parse_object<'a>(buf: &mut Cursor<&'a [u8]>) -> MsgObject<'a> {
    match read_marker(buf) {
        Ok(Marker::FixArray(num)) => parse_array(num, buf),
        Ok(Marker::Bin8) | Ok(Marker::Bin32) => parse_bin(buf),
        Ok(Marker::FixMap(num)) => parse_map(num, buf),
        Ok(Marker::U32) => parse_u32(buf),
        Ok(Marker::U16) => parse_u16(buf),
        Ok(Marker::True) => MsgObject::Bool(true),
        err => unimplemented!("can not parse msg pack {:?}", err),
    }
}

#[inline]
fn parse_u32<'a>(buf: &mut Cursor<&'a [u8]>) -> MsgObject<'a> {
    let pos = buf.position() as usize;
    let bin = &buf.get_ref()[pos..pos + 4];
    buf.seek(SeekFrom::Current(4 as i64)).unwrap();
    MsgObject::UInt(bin)
}

#[inline]
fn parse_u16<'a>(buf: &mut Cursor<&'a [u8]>) -> MsgObject<'a> {
    let pos = buf.position() as usize;
    let bin = &buf.get_ref()[pos..pos + 2];
    buf.seek(SeekFrom::Current(2 as i64)).unwrap();
    MsgObject::UInt(bin)
}

#[inline]
fn parse_bin<'a>(buf: &mut Cursor<&'a [u8]>) -> MsgObject<'a> {
    buf.seek(SeekFrom::Current(-1)).unwrap();
    let size = read_bin_len(buf).unwrap();
    let pos = buf.position() as usize;
    let bin = &buf.get_ref()[pos..pos + size as usize];
    buf.seek(SeekFrom::Current(size as i64)).unwrap();
    MsgObject::Bin(bin)
}

#[inline]
fn parse_map<'a>(num: u8, buf: &mut Cursor<&'a [u8]>) -> MsgObject<'a> {
    let mut ret = HashMap::new();
    for _ in 0..num {
        let key_obj = &mut parse_object(buf);
        let key = {
            match key_obj {
                MsgObject::Bin(bytes) => std::str::from_utf8(bytes).unwrap(),
                err => unimplemented!("Unsupported key type {:?}", err),
            }
        };
        let val = parse_object(buf);
        ret.insert(key, Box::new(val));
    }
    MsgObject::Map(ret)
}

#[inline]
fn parse_array<'a>(num: u8, buf: &mut Cursor<&'a [u8]>) -> MsgObject<'a> {
    let mut ret = Vec::with_capacity(num as usize);
    for _ in 0..num {
        ret.push(Box::new(parse_object(buf)));
    }
    MsgObject::Array(ret)
}

pub fn msg_unpack<'a>(bytes: &'a [u8]) -> Vec<MsgObject<'a>> {
    let mut ret = Vec::new();
    let mut buf = Cursor::new(bytes);
    loop {
        if buf.is_empty() {
            break;
        }
        ret.push(parse_object(&mut buf));
    }
    ret
}

fn get_array_size(vec: &Vec<Box<MsgObject>>) -> usize {
    let mut size = get_array_head_size(vec.len());
    for obj in vec {
        size += msg_size(obj.as_ref());
    }
    size
}

pub fn get_array_head_size(len: usize) -> usize {
    match len {
        0..=15 => 1,
        16..=U8_MAX => 2,
        err => unimplemented!("can not encode vec with size {:?}", err),
    }
}

fn get_bin_size(bin: &[u8]) -> usize {
    let l = bin.len();
    get_bin_size_from_len(l)
}

pub fn get_bin_size_from_len(len: usize) -> usize {
    match len {
        1..=U8_MAX => 2 + len,
        1..=U16_MAX => 3 + len,
        1..=U32_MAX => 5 + len,
        err => unimplemented!("can not encode bin with size {:?}", err),
    }
}

pub fn msg_size(object: &MsgObject) -> usize {
    match object {
        MsgObject::Array(vec) => get_array_size(vec),
        MsgObject::Bin(bin) => get_bin_size(*bin),
        MsgObject::UInt(bin) => bin.len() + 1,
        err => unimplemented!("can not parse msg pack {:?}", err),
    }
}

fn write_array(w: &mut Cursor<&mut [u8]>, vec: &Vec<Box<MsgObject>>) {
    write_array_len(w, vec.len() as u32).unwrap();
    for obj in vec {
        write_object(w, obj.as_ref());
    }
}

pub fn write_int(w: &mut Cursor<&mut [u8]>, bin: &[u8]) {
    println!("{:}", w.position());
    match bin.len() {
        1 => write_u8(w, u8::from_be_bytes(bin.try_into().unwrap())).unwrap(),
        2 => write_u16(w, u16::from_be_bytes(bin.try_into().unwrap())).unwrap(),
        4 => write_u32(w, u32::from_be_bytes(bin.try_into().unwrap())).unwrap(),
        err => unimplemented!("can not parse msg pack {:?}", err),
    };
}

fn write_object(w: &mut Cursor<&mut [u8]>, object: &MsgObject) {
    match object {
        MsgObject::Array(vec) => write_array(w, vec),
        MsgObject::Bin(bin) => write_bin(w, *bin).unwrap(),
        MsgObject::UInt(bin) => write_int(w, *bin),
        err => unimplemented!("can not parse msg pack {:?}", err),
    }
}

pub fn msgpack(bytes: &mut [u8], object: &MsgObject) {
    let mut writer = Cursor::new(bytes);
    write_object(&mut writer, object);
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{jpeg::JpegDecoder, ImageDecoder};
    use lmdb::open::{NOSUBDIR, RDONLY};
    use lmdb_zero as lmdb;
    #[test]
    fn test_decode() {
        let location = "/home/xiej/data/lmdb-imagenet/ILSVRC-train.lmdb";
        let env = unsafe {
            lmdb::EnvBuilder::new()
                .unwrap()
                .open(location, RDONLY | NOSUBDIR, 0o600)
                .unwrap()
        };
        let db = lmdb::Database::open(&env, None, &lmdb::DatabaseOptions::defaults()).unwrap();
        let txn = lmdb::ReadTransaction::new(&env).unwrap();
        let acc = txn.access();
        let data: &[u8] = acc.get(&db, 0.to_string().as_bytes()).unwrap();
        let mut data = msg_unpack(data);
        let image;
        match &mut data[0] {
            MsgObject::Array(data) => {
                match data[0].as_mut() {
                    MsgObject::Map(map) => {
                        println!("{:?}", map.keys());
                        image = map.get_mut("data");
                    }
                    _ => unreachable!(),
                };
            }
            _ => unreachable!(),
        };

        let image = image.unwrap();
        if let MsgObject::Bin(bytes) = image.as_ref() {
            println!("{}", bytes.len());
            let x = JpegDecoder::new(Cursor::new(bytes)).unwrap();
            println!("{:?} color_type {:?} bytes_num {}", x.dimensions(), x.original_color_type(), x.total_bytes());
            let mut vec = vec![0u8; 499500];
            println!("{:?}", x.read_image(&mut vec));
        }
    }

    #[test]
    fn test_encode() {
        let int = &2u16.to_be_bytes();
        let bin = &[7u8; 16];
        let msg_int = Box::new(MsgObject::UInt(int));
        let msg_bin = Box::new(MsgObject::Bin(bin));
        let array = MsgObject::Array(vec![msg_bin, msg_int]);
        let size = msg_size(&array);
        assert_eq!(size, 1 + 18 + 3);
        let mut write = vec![0u8; size];
        msgpack(&mut write, &array);
        assert_eq!(&write[3..19], bin);
        assert_eq!(&write[20..], int);
        let b = &mut [0u8; 3];
        write_int(&mut Cursor::new(b), &2u16.to_be_bytes());
        assert_eq!(&b[1..], &2u16.to_be_bytes());
    }
}
