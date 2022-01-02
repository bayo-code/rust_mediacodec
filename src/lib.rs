mod codec;
mod crypto;
mod error;
mod extractor;
mod format;
mod native_window;
mod samples;

pub use codec::*;
pub use crypto::*;
pub use error::*;
pub use extractor::*;
pub use format::*;
use jni::objects::JObject;
use log::*;
pub use native_window::*;
pub use samples::*;

use jni::sys::JNI_VERSION_1_6;
use jni::{JNIEnv, JavaVM};

#[no_mangle]
pub extern "C" fn JNI_OnLoad(_vm: JavaVM, _version: i32) -> i32 {
    javavm::set_jvm(Some(_vm));
    android_log::init("rust_mediacodec").unwrap();

    return JNI_VERSION_1_6;
}

#[no_mangle]
extern "C" fn Java_tech_smallwonder_mydiary_MainActivity_process(_: JNIEnv, _: JObject) {
    let original = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        original(info);
        error!("Panic: {}", info.to_string());
        // std::process::abort();
    }));

    let mut extractor =
        MediaExtractor::from_url("/storage/emulated/0/Yahir Marca Me Lapiel 2.mp3").unwrap();

    debug!("Track count: {}", extractor.track_count());

    let mut decoders = vec![];

    for i in 0..extractor.track_count() {
        let format = extractor.track_format(i);
        debug!("{}", format.to_string());
        let mime_type = format.get_string("mime").unwrap();
        let mut codec = MediaCodec::create_decoder(&mime_type).unwrap();
        codec.init(&format, None, 0).unwrap();
        codec.start().unwrap();
        decoders.push(codec);
        extractor.select_track(i);
    }

    while extractor.has_next() {
        // 1. Get the track index
        let index = extractor.track_index();

        if index < 0 {
            break;
        }

        let codec = &mut decoders[index as usize];

        // Fetch the codec's input buffer
        while let Ok(mut buffer) = codec.dequeue_input() {
            debug!("Got input buffer with index: {}", buffer.index());
            if !extractor.read_next(&mut buffer) {
                debug!("MediaExtractor.read_next() returned false!");
            }

            // Hopefully, when the buffer gets dropped (here), the buffer will be queued back to MediaCodec
            // And we don't have to explicitly return anything
        }

        // Check for output
        let output_fmt = codec.output_format().unwrap();
        while let Ok(mut buffer) = codec.dequeue_output() {
            debug!("Got output buffer with index: {}", buffer.index());
            // Normally, we'd do this for video codecs we have used surfaces for
            let info = buffer.info();
            debug!("Output buffer info: {info:?}");
            debug!("Output format: {}", output_fmt.to_string());
            buffer.set_render(true);
        }
    }
}
