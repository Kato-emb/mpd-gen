use std::io::Write;

use mpdgen::MPD;
use serde::Serialize;

fn main() {
    let data = std::fs::read("manifest.mpd").unwrap();
    let mpd = String::from_utf8(data).unwrap();
    let mpd: MPD = quick_xml::de::from_str(&mpd).unwrap();
    // Missing attribute @xsi:chemaLocation
    println!("{:?}", mpd);

    let mut xml = String::new();
    let mut ser = quick_xml::se::Serializer::new(&mut xml);
    ser.indent(' ', 2);
    mpd.serialize(ser).unwrap();

    let mut file = std::fs::File::create("manifest_copy.mpd").unwrap();
    file.write_all(xml.as_bytes()).unwrap();
}
