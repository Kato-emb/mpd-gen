use std::io::{Cursor, Write};

use mpdgen::MPD;
use serde::Serialize;

fn main() {
    let data = std::fs::read("manifest.mpd").unwrap();
    let mut reader = Cursor::new(data);
    let mpd = MPD::read(&mut reader).unwrap();
    // Missing attribute @xsi:chemaLocation
    println!("{:?}", mpd);

    let mut xml = String::new();
    let mut ser = quick_xml::se::Serializer::new(&mut xml);
    ser.indent(' ', 2);
    mpd.serialize(ser).unwrap();

    let mut file = std::fs::File::create("manifest_copy.mpd").unwrap();
    file.write_all(xml.as_bytes()).unwrap();
}
