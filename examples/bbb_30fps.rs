use std::{
    io::Write,
    path::{Path, PathBuf},
    str::FromStr,
};

use axum::{routing::get_service, Router};
use mpd_gen::{
    AdaptationSetBuilder, BaseURLBuilder, Codecs, ContentType, DescriptorBuilder, MPDBuilder,
    PeriodBuilder, PresentationType, Profile, Ratio, RepresentationBuilder, SegmentTemplateBuilder,
    StreamAccessPoint, StringNoWhitespace, VideoScan,
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let tmp_dir = PathBuf::from("public");
    if !tmp_dir.exists() {
        std::fs::create_dir_all(&tmp_dir)?;
    }

    let video_segment_template = SegmentTemplateBuilder::default()
        .duration(120u32)
        .timescale(30u32)
        .media("$RepresentationID$/$RepresentationID$_$Number$.m4v")
        .start_number(1u32)
        .initialization_attribute("$RepresentationID$/$RepresentationID$_0.m4v")
        .build()?;

    let repr_576p_2500k = RepresentationBuilder::default()
        .id(StringNoWhitespace::from_str("bbb_30fps_1024x576_2500k")?)
        .codecs(Codecs::from_str("avc1.64001f")?)
        .bandwidth(3134488u32)
        .width(1024u32)
        .height(576u32)
        .framerate(30u32)
        .sar(Ratio::from_str("1:1")?)
        .scan_type(VideoScan::Progressive)
        .build()?;

    let repr_720p_4000k = RepresentationBuilder::default()
        .id(StringNoWhitespace::from_str("bbb_30fps_1280x720_4000k")?)
        .codecs(Codecs::from_str("avc1.64001f")?)
        .bandwidth(4952892u32)
        .width(1280u32)
        .height(720u32)
        .framerate(30u32)
        .sar(Ratio::from_str("1:1")?)
        .scan_type(VideoScan::Progressive)
        .build()?;

    let repr_1080p_8000k = RepresentationBuilder::default()
        .id(StringNoWhitespace::from_str("bbb_30fps_1920x1080_8000k")?)
        .codecs(Codecs::from_str("avc1.640028")?)
        .bandwidth(9914554u32)
        .width(1920u32)
        .height(1080u32)
        .framerate(30u32)
        .sar(Ratio::from_str("1:1")?)
        .scan_type(VideoScan::Progressive)
        .build()?;

    let repr_180p_200k = RepresentationBuilder::default()
        .id(StringNoWhitespace::from_str("bbb_30fps_320x180_200k")?)
        .codecs(Codecs::from_str("avc1.64000d")?)
        .bandwidth(254320u32)
        .width(320u32)
        .height(180u32)
        .framerate(30u32)
        .sar(Ratio::from_str("1:1")?)
        .scan_type(VideoScan::Progressive)
        .build()?;

    let repr_180p_400k = RepresentationBuilder::default()
        .id(StringNoWhitespace::from_str("bbb_30fps_320x180_400k")?)
        .codecs(Codecs::from_str("avc1.64000d")?)
        .bandwidth(507246u32)
        .width(320u32)
        .height(180u32)
        .framerate(30u32)
        .sar(Ratio::from_str("1:1")?)
        .scan_type(VideoScan::Progressive)
        .build()?;

    let repr_270p_600k = RepresentationBuilder::default()
        .id(StringNoWhitespace::from_str("bbb_30fps_480x270_600k")?)
        .codecs(Codecs::from_str("avc1.640015")?)
        .bandwidth(759798u32)
        .width(480u32)
        .height(270u32)
        .framerate(30u32)
        .sar(Ratio::from_str("1:1")?)
        .scan_type(VideoScan::Progressive)
        .build()?;

    let repr_360p_1000k = RepresentationBuilder::default()
        .id(StringNoWhitespace::from_str("bbb_30fps_640x360_1000k")?)
        .codecs(Codecs::from_str("avc1.64001e")?)
        .bandwidth(1254758u32)
        .width(640u32)
        .height(360u32)
        .framerate(30u32)
        .sar(Ratio::from_str("1:1")?)
        .scan_type(VideoScan::Progressive)
        .build()?;

    let repr_360p_800k = RepresentationBuilder::default()
        .id(StringNoWhitespace::from_str("bbb_30fps_640x360_800k")?)
        .codecs(Codecs::from_str("avc1.64001e")?)
        .bandwidth(1013310u32)
        .width(640u32)
        .height(360u32)
        .framerate(30u32)
        .sar(Ratio::from_str("1:1")?)
        .scan_type(VideoScan::Progressive)
        .build()?;

    let repr_432p_1500k = RepresentationBuilder::default()
        .id(StringNoWhitespace::from_str("bbb_30fps_768x432_1500k")?)
        .codecs(Codecs::from_str("avc1.64001e")?)
        .bandwidth(1883700u32)
        .width(768u32)
        .height(432u32)
        .framerate(30u32)
        .sar(Ratio::from_str("1:1")?)
        .scan_type(VideoScan::Progressive)
        .build()?;

    let repr_2160p_12000k = RepresentationBuilder::default()
        .id(StringNoWhitespace::from_str("bbb_30fps_3840x2160_12000k")?)
        .codecs(Codecs::from_str("avc1.640033")?)
        .bandwidth(14931538u32)
        .width(3840u32)
        .height(2160u32)
        .framerate(30u32)
        .sar(Ratio::from_str("1:1")?)
        .scan_type(VideoScan::Progressive)
        .build()?;

    let video_adapt = AdaptationSetBuilder::default()
        .mime_type("video/mp4")
        .content_type(ContentType::Video)
        .subsegment_alignment(true)
        .subsegment_starts_with_sap(StreamAccessPoint::try_from(1)?)
        .par(Ratio::from_str("16:9")?)
        .segment_template(video_segment_template)
        .representation(&[
            repr_576p_2500k,
            repr_720p_4000k,
            repr_1080p_8000k,
            repr_180p_200k,
            repr_180p_400k,
            repr_270p_600k,
            repr_360p_1000k,
            repr_360p_800k,
            repr_432p_1500k,
            repr_2160p_12000k,
        ])
        .build()?;

    let audio_segment_template = SegmentTemplateBuilder::default()
        .duration(192512u32)
        .timescale(48000u32)
        .media("$RepresentationID$/$RepresentationID$_$Number$.m4a")
        .start_number(1u32)
        .initialization_attribute("$RepresentationID$/$RepresentationID$_0.m4a")
        .build()?;

    let repr_a64k = RepresentationBuilder::default()
        .id(StringNoWhitespace::from_str("bbb_a64k")?)
        .codecs(Codecs::from_str("mp4a.40.5")?)
        .bandwidth(67071u32)
        .audio_sampling_rate(vec![48000u32])
        .audio_channel_configuration(&[DescriptorBuilder::default()
            .scheme_id_uri("urn:mpeg:dash:23003:3:audio_channel_configuration:2011")
            .value("2")
            .build()?])
        .build()?;

    let audio_adapt = AdaptationSetBuilder::default()
        .mime_type("audio/mp4")
        .content_type(ContentType::Audio)
        .subsegment_alignment(true)
        .subsegment_starts_with_sap(StreamAccessPoint::try_from(1)?)
        .accessibility(&[DescriptorBuilder::default()
            .scheme_id_uri("urn:tva:metadata:cs:AudioPurposeCS:2007")
            .value("6")
            .build()?])
        .role(&[DescriptorBuilder::default()
            .scheme_id_uri("urn:mpeg:dash:role:2011")
            .value("main")
            .build()?])
        .segment_template(audio_segment_template)
        .representation(&[repr_a64k])
        .build()?;

    let period = PeriodBuilder::default()
        .adaptation_set(&[video_adapt, audio_adapt])
        .build()?;

    let mpd = MPDBuilder::default()
        .media_presentation_duration(std::time::Duration::from_millis(634566))
        .min_buffer_time(std::time::Duration::from_secs(2))
        .profiles(vec![
            Profile::IsoLive,
            Profile::Other("urn:hbbtv:dash:profile:isoff-live:2012".to_string()),
        ])
        .r#type(PresentationType::Static)
        .xmlns("urn:mpeg:dash:schema:mpd:2011")
        .xmlns_xsi("http://www.w3.org/2001/XMLSchema-instance")
        .xsi_schema_location(vec!["urn:mpeg:DASH:schema:MPD:2011", "DASH-MPD.xsd"])
        .base_url(&[BaseURLBuilder::default()
            .base("https://dash.akamaized.net/akamai/bbb_30fps/")
            .build()?])
        .period(&[period])
        .build()?;

    let mpd_file_path = tmp_dir.join("manifest.mpd");
    let mut file = std::fs::File::create(&mpd_file_path)?;
    mpd.write(&mut file)?;

    let server = tokio::task::spawn(server_init(tmp_dir.clone()));

    tokio::signal::ctrl_c().await?;
    println!("Received Ctrl+C, shutting down...");

    server.abort();
    std::fs::remove_dir_all(tmp_dir)?;

    Ok(())
}

async fn server_init<P: AsRef<Path>>(path: P) -> Result<(), std::io::Error> {
    let content = r##"<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>DASH.js Minimal Example</title>
    <script src="https://cdn.dashjs.org/latest/dash.all.min.js"></script>
</head>

<body>
    <video id="videoPlayer" controls></video>
    <div>
        <span id="min"></span><span id="sec"></span>
    </div>
    <script>
        function init() {
            var video,
                player,
                url = 'http://0.0.0.0:8000/manifest.mpd';

            video = document.querySelector("#videoPlayer");
            player = dashjs.MediaPlayer().create();

            player.updateSettings({ 'debug': { 'logLevel': dashjs.Debug.LOG_LEVEL_DEBUG } });
            player.initialize(video, url, true);
        }

        window.onload = init;
    </script>
</body>

</html>
"##;

    let html_file_path = path.as_ref().join("index.html");
    let mut file = std::fs::File::create(&html_file_path)?;
    file.write_all(content.as_bytes())?;

    let app = Router::new().fallback(get_service(tower_http::services::ServeDir::new(path)));

    println!("Web server started: http://0.0.0.0:8000");

    axum_server::bind("0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
}
