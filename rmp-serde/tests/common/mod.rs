use std::io::Cursor;

use serde::Deserialize;

use rmp_serde::Deserializer;

mod de;
mod se;

#[test]
fn pass_deserializer_get_ref() {
    let buf = [0xc0];
    let cur = Cursor::new(&buf[..]);

    let mut de = Deserializer::from_read(cur);

    assert_eq!((), Deserialize::deserialize(&mut de).ok().unwrap());
    assert_eq!(1, de.get_ref().rd.position());
}

#[test]
fn pass_deserializer_get_mut() {
    let buf = [0xc0];
    let cur = Cursor::new(&buf[..]);

    let mut de = Deserializer::from_read(cur);

    assert_eq!((), Deserialize::deserialize(&mut de).ok().unwrap());
    de.get_mut().rd.set_position(0);

    assert_eq!((), Deserialize::deserialize(&mut de).ok().unwrap());
}

#[test]
fn pass_deserializer_into_inner() {
    let buf = [0xc0];
    let cur = Cursor::new(&buf[..]);

    let mut de = Deserializer::from_read(cur);

    assert_eq!((), Deserialize::deserialize(&mut de).ok().unwrap());
    let cur = de.into_inner();

    assert_eq!(1, cur.rd.position());
}
