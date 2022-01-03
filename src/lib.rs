mod codec;
mod crypto;
mod error;
mod extractor;
mod format;
mod native_window;
mod samples;

use std::sync::{Arc, Mutex};

pub use codec::*;
use cpal::traits::DeviceTrait;
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

// Java_tech_smallwonder_mydiary_MainActivity_process

#[no_mangle]
extern "C" fn process() {
    let original = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        original(info);
        error!("Panic: {}", info.to_string());
        // std::process::abort();
    }));

    let pool: Arc<Mutex<Vec<i16>>> = Arc::new(Mutex::new(Vec::new()));
    let other_pool = Arc::clone(&pool);

    use cpal::traits::HostTrait;

    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .expect("failed to find output device");

    let config = device.default_output_config().unwrap();

    debug!("Config: {:?}", config);

    match config.sample_format() {
        cpal::SampleFormat::I16 => {
            debug!("Found I16");
        }
        _ => {
            debug!("Some other audio fmt!");
        }
    }

    let stream = device
        .build_output_stream(
            &config.config(),
            move |data: &mut [i16], _: &cpal::OutputCallbackInfo| {},
            move |err| {},
        )
        .unwrap();

    return;

    let mut extractor =
        MediaExtractor::from_url("/storage/emulated/0/Yahir Marca Me Lapiel 2.mp3").unwrap();

    debug!("Track count: {}", extractor.track_count());

    let fmt = extractor.track_format(0).unwrap();

    let mut decoders = vec![];

    for i in 0..extractor.track_count() {
        let format = extractor.track_format(i).unwrap();
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
            // debug!("Got input buffer with index: {}", buffer.index());
            if !extractor.read_next(&mut buffer) {
                debug!("MediaExtractor.read_next() returned false!");
                break; // VERY IMPORTANT, there's nothing else to DO!!!
            }

            // Hopefully, when the buffer gets dropped (here), the buffer will be queued back to MediaCodec
            // And we don't have to explicitly return anything
        }

        // Check for output
        let output_fmt = codec.output_format().unwrap();
        while let Ok(mut buffer) = codec.dequeue_output() {
            // debug!("Got output buffer with index: {}", buffer.index());
            // Normally, we'd do this for video codecs we have used surfaces for
            let info = buffer.info();
            // debug!("Output buffer info: {info:?}");
            debug!("Output format: {}", output_fmt.to_string());

            if let Some(ref frame) = buffer.frame() {
                match frame {
                    Frame::Audio(value) => match value.format() {
                        SampleFormat::S16(buf) => {
                            let mut guard = pool.lock().unwrap();
                            // Resize this to make sure it accommodates the new data
                            guard.extend(*buf);
                        }
                        SampleFormat::F32(_) => {}
                    },
                    Frame::Video(_) => todo!(),
                }
            }

            buffer.set_render(true);
        }
    }
}

#[ndk_glue::main(backtrace = "on", logger(level = "debug", tag = "rust_mediacodec"))]
fn main() {
    let activity = ndk_glue::native_activity();
    javavm::set_jvm(Some(unsafe { JavaVM::from_raw(activity.vm()) }.unwrap()));

    javavm::jvm()
        .unwrap()
        .attach_current_thread_as_daemon()
        .unwrap();

    // process();
}
