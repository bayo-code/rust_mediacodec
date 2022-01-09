//! This crate provides bindings to the MediaCodec APIs in the Android NDK.
//!
//! Examples:
//! ### Decoding
//! ```edition2021
//!
//! # #[no_mangle]
//! # extern "C" fn process() {
//!     use mediacodec::{Frame, MediaCodec, MediaExtractor, SampleFormat, VideoFrame};
//!     let mut extractor = MediaExtractor::from_url("/path/to/a/resource").unwrap();

//!     debug!("Track count: {}", extractor.track_count());

//!     let mut decoders = vec![];

//!     for i in 0..extractor.track_count() {
//!         let format = extractor.track_format(i).unwrap();
//!         debug!("{}", format.to_string());
//!         let mime_type = format.get_string("mime").unwrap();
//!         let mut codec = MediaCodec::create_decoder(&mime_type).unwrap();

//!         codec.init(&format, None, 0).unwrap();

//!         codec.start().unwrap();
//!         decoders.push(codec);
//!         extractor.select_track(i);
//!     }

//!     while extractor.has_next() {
//!         // 1. Get the track index
//!         let index = extractor.track_index();
//!
//!         if index < 0 {
//!             break;
//!         }
//!
//!         let codec = &mut decoders[index as usize];
//!
//!         // Fetch the codec's input buffer
//!         while let Ok(mut buffer) = codec.dequeue_input() {
//!             if !extractor.read_next(&mut buffer) {
//!                 debug!(
//!                     "MediaExtractor.read_next() returned false! has_next(): {}",
//!                     extractor.has_next()
//!                 );
//!                 break; // VERY IMPORTANT, there's nothing else to DO, so break!!!
//!             }
//!
//!             // When the buffer gets dropped (here), the buffer will be queued back to MediaCodec
//!             // And we don't have to do anything else
//!         }
//!
//!         // Check for output
//!         let output_fmt = codec.output_format().unwrap();
//!         while let Ok(mut buffer) = codec.dequeue_output() {
//!             if let Some(ref frame) = buffer.frame() {
//!                 match frame {
//!                     Frame::Audio(value) => match value.format() {
//!                         SampleFormat::S16(_) => {
//!                             // Do something with the audio frame
//!                         }
//!                         SampleFormat::F32(_) => {
//!                             // Do something with the audio frame
//!                         }
//!                     },
//!                     Frame::Video(value) => match value {
//!                         VideoFrame::Hardware => {
//!                             // Nothing TODO. The frame will be rendered
//!                         }
//!                         VideoFrame::RawFrame(_) => {
//!                             // Read out the raw buffers or something
//!                         }
//!                     },
//!                 }
//!             }
//!
//!             // Set the buffer to render when dropped. Only applicable to video codecs that have a hardware buffer (i.e, attached to a native window)
//!             buffer.set_render(true);
//!         }
//!     }
//! # }
//! ```
//!
//! ### Demuxing
//! ```edition2021
//! use log::debug;
//! use mediacodec::{Frame, MediaExtractor, SampleFormat, VideoFrame};
//!
//! # #[no_mangle]
//! # extern "C" fn process() {
//!     let mut extractor = MediaExtractor::from_url("/path/to/a/resource").unwrap();
//!
//!     debug!("Track count: {}", extractor.track_count());
//!
//!     for i in 0..extractor.track_count() {
//!         let format = extractor.track_format(i).unwrap();
//!         debug!("{}", format.to_string());
//!         let mime_type = format.get_string("mime").unwrap();
//!         extractor.select_track(i);
//!     }
//!
//!     while extractor.has_next() {
//!         // 1. Get the track index
//!         let index = extractor.track_index();
//!
//!         if index < 0 {
//!             break;
//!         }
//!
//!         // Get a codec buffer and read data into it
//!     }
//! # }

//! ```
// #![cfg(os = "android")]

mod codec;
mod crypto;
mod error;
mod extractor;
mod format;
mod muxer;
mod native_window;
mod samples;

pub use codec::*;
pub use crypto::*;
pub use error::*;
pub use extractor::*;
pub use format::*;
pub use muxer::*;
pub use native_window::*;
pub use samples::*;
