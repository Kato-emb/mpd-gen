use std::{io::Write, str::FromStr};

use mpdgen::{
    AdaptationSetBuilder, BaseURLBuilder, Codecs, ContentType, DescriptorBuilder, MPDBuilder,
    NoWhitespace, PeriodBuilder, PresentationType, Profile, RepresentationBuilder, SegmentBuilder,
    SegmentTemplateBuilder, SegmentTimelineBuilder, StreamAccessPoint, VideoScan,
    DASH_DVB_EXTENTION, ROLE_SCHEME,
};
use serde::Serialize;

fn main() {
    let segment = SegmentBuilder::default()
        .duration(15000u64)
        .repeat_count(999)
        .build()
        .unwrap();
    let segment_timeline = SegmentTimelineBuilder::default()
        .segment(segment)
        .build()
        .unwrap();

    let segment_template = SegmentTemplateBuilder::default()
        .initialization_attribute("$RepresentationID$.cmfi")
        .media("$RepresentationID$/$Time$.cmfv")
        .timescale(3000u32)
        .segment_timeline(segment_timeline)
        .build()
        .unwrap();

    let repr = RepresentationBuilder::default()
        .id(NoWhitespace::from_str("720p").unwrap())
        .codecs(Codecs::from_str("avc1.4d0028").unwrap())
        .bandwidth(4_000_000u32)
        .width(1280u32)
        .height(720u32)
        .scan_type(VideoScan::Progressive)
        .dependency_id(vec!["a", "b"])
        .build()
        .unwrap();

    let role = DescriptorBuilder::default()
        .scheme_id_uri(ROLE_SCHEME)
        .value("main")
        .build()
        .unwrap();

    let adapt = AdaptationSetBuilder::default()
        .content_type(ContentType::Video)
        .segment_alignment(true)
        .mime_type("video/mp4")
        .start_with_sap(StreamAccessPoint::Type1)
        .role(vec![role])
        .representation(vec![repr])
        .segment_template(segment_template)
        .build()
        .unwrap();

    let base_url = BaseURLBuilder::default().base("./").build().unwrap();

    let period = PeriodBuilder::default()
        .start(std::time::Duration::from_secs(0))
        .duration(std::time::Duration::from_secs(5000))
        .adaptation_set(vec![adapt])
        .base_url(vec![base_url])
        .build()
        .unwrap();

    let mpd = MPDBuilder::default()
        .xmlns_dvb(DASH_DVB_EXTENTION)
        .r#type(PresentationType::Dynamic)
        .min_buffer_time(std::time::Duration::from_secs(5))
        .publish_time(chrono::Utc::now())
        .availability_start_time(chrono::Utc::now())
        .minimum_undate_period(std::time::Duration::from_secs(5))
        .profiles(vec![Profile::Cmaf])
        .period(vec![period])
        .build()
        .unwrap();

    println!("{:?}", mpd);

    let mut xml = String::new();
    let mut ser = quick_xml::se::Serializer::new(&mut xml);
    ser.indent(' ', 2);
    mpd.serialize(ser).unwrap();

    let xml = format!("<?xml version=\"1.0\" encoding=\"utf-8\">\n{xml}");

    let mut file = std::fs::File::create("manifest.mpd").unwrap();
    file.write_all(xml.as_bytes()).unwrap();
}
