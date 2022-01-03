mod codec;
mod crypto;
mod error;
mod extractor;
mod format;
mod native_window;
mod samples;

use std::sync::{Arc, Mutex};

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

// Java_tech_smallwonder_mydiary_MainActivity_process

#[no_mangle]
extern "C" fn process() {
    let original = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        original(info);
        error!("Panic: {}", info.to_string());
        // std::process::abort();
    }));

    let pool: Arc<Mutex<VecDeque<i16>>> = Arc::new(Mutex::new(VecDeque::new()));
    let other_pool = Arc::clone(&pool);

    use oboe::{
        AudioOutputCallback, AudioOutputStream, AudioStream, AudioStreamBuilder,
        DataCallbackResult, IsFrameType, PerformanceMode, SharingMode, Stereo,
    };
    use std::collections::VecDeque;

    struct AudioOutput {
        pool: Arc<Mutex<VecDeque<i16>>>,
    }

    impl AudioOutputCallback for AudioOutput {
        type FrameType = (i16, Stereo);

        fn on_audio_ready(
            &mut self,
            audio_stream: &mut dyn oboe::AudioOutputStreamSafe,
            audio_data: &mut [(i16, i16)],
        ) -> DataCallbackResult {
            let mut buf = self.pool.lock().unwrap();
            // debug!("Callback... Buf len: {}", buf.len());
            for i in 0..audio_data.len() {
                let value1 = buf.pop_front();
                if let Some(value) = value1 {
                    let value2 = buf.pop_front().unwrap();

                    audio_data[i] = (value, value2);
                } else {
                    break;
                }
            }
            DataCallbackResult::Continue
        }
    }

    // return;

    let mut extractor = MediaExtractor::from_url(
        "/storage/emulated/0/Eminem_Ft_Cordae_Jack_Harlow_-_Killer_Remix_.mp3",
    )
    .unwrap();

    debug!("Track count: {}", extractor.track_count());

    let fmt = extractor.track_format(0).unwrap();

    let mut decoders = vec![];

    std::thread::sleep(std::time::Duration::from_millis(400));

    let mut sample_rate = 0;

    for i in 0..extractor.track_count() {
        let format = extractor.track_format(i).unwrap();
        debug!("{}", format.to_string());
        let mime_type = format.get_string("mime").unwrap();
        let mut codec = MediaCodec::create_decoder(&mime_type).unwrap();

        if format.is_audio() {
            sample_rate = format.get_i32("sample-rate").unwrap();

            codec.init(&format, None, 0).unwrap();
        } else {
            // We assume it's video here
            let window: *mut ANativeWindow = ndk_glue::native_window()
                .as_ref()
                .unwrap()
                .ptr()
                .as_ptr()
                .cast();
            let window = NativeWindow::from_raw(window);
            codec.init(&format, Some(window), 0).unwrap();
        }
        codec.start().unwrap();
        decoders.push(codec);
        extractor.select_track(i);
    }

    let mut audio = AudioStreamBuilder::default()
        .set_performance_mode(PerformanceMode::LowLatency)
        .set_sharing_mode(SharingMode::Shared)
        .set_format::<i16>()
        .set_channel_count::<Stereo>()
        .set_sample_rate(sample_rate)
        .set_callback(AudioOutput { pool: other_pool })
        .open_stream()
        .unwrap();

    debug!("Calling audio.start...");
    audio.start().unwrap();

    while extractor.has_next() {
        // 1. Get the track index
        let index = extractor.track_index();

        if index < 0 {
            break;
        }

        let codec = &mut decoders[index as usize];

        // Fetch the codec's input buffer
        while let Ok(mut buffer) = codec.dequeue_input() {
            if !extractor.read_next(&mut buffer) {
                debug!(
                    "MediaExtractor.read_next() returned false! has_next(): {}",
                    extractor.has_next()
                );
                break; // VERY IMPORTANT, there's nothing else to DO, so break!!!
            }

            // When the buffer gets dropped (here), the buffer will be queued back to MediaCodec
            // And we don't have to do anything else
        }

        // Check for output
        let output_fmt = codec.output_format().unwrap();
        while let Ok(mut buffer) = codec.dequeue_output() {
            // Normally, we'd do this for video codecs we have used surfaces for
            let info = buffer.info();

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
                    Frame::Video(value) => match value {
                        VideoFrame::Hardware => {
                            // Nothing TODO. The frame will be rendered
                        }
                        VideoFrame::RawFrame(_) => todo!(),
                    },
                }
            }

            buffer.set_render(true);
        }
    }

    loop {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}

#[ndk_glue::main(backtrace = "on", logger(level = "debug", tag = "rust_mediacodec"))]
fn main() {
    // android_log::init("rust_mediacodec").unwrap();
    let activity = ndk_glue::native_activity();
    javavm::set_jvm(Some(unsafe { JavaVM::from_raw(activity.vm()) }.unwrap()));

    javavm::jvm()
        .unwrap()
        .attach_current_thread_as_daemon()
        .unwrap();

    process();
}
