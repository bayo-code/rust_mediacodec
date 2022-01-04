use log::debug;
use mediacodec::{Frame, MediaExtractor, SampleFormat, VideoFrame};

#[no_mangle]
extern "C" fn process() {
    let mut extractor = MediaExtractor::from_url("/path/to/a/resource").unwrap();

    debug!("Track count: {}", extractor.track_count());

    for i in 0..extractor.track_count() {
        let format = extractor.track_format(i).unwrap();
        debug!("{}", format.to_string());
        let mime_type = format.get_string("mime").unwrap();
        extractor.select_track(i);
    }

    while extractor.has_next() {
        // 1. Get the track index
        let index = extractor.track_index();

        if index < 0 {
            break;
        }

        // Get a codec buffer and read data into it
    }
}
