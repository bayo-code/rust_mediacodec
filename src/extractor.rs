use std::{
    ffi::{CStr, CString},
    marker::PhantomData,
    os::raw::c_char,
};

use log::{debug, info};

use crate::{AMediaFormat, CodecInputBuffer, MediaFormat, MediaStatus};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct AMediaExtractor {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[link(name = "mediandk")]
extern "C" {
    /// Since: API 21
    fn AMediaExtractor_new() -> *mut AMediaExtractor;

    /// Since: API 21
    fn AMediaExtractor_delete(extractor: *mut AMediaExtractor) -> isize;

    /// Since: API 21
    fn AMediaExtractor_setDataSourceFd(
        extractor: *mut AMediaExtractor,
        fd: i32,
        offset: u64,
        length: u64,
    ) -> isize;

    /// Since: API 21
    fn AMediaExtractor_setDataSource(
        extractor: *mut AMediaExtractor,
        location: *const c_char,
    ) -> isize;

    /// Since: API 21
    fn AMediaExtractor_getTrackCount(extractor: *mut AMediaExtractor) -> usize;

    /// Since: API 21
    fn AMediaExtractor_getTrackFormat(
        extractor: *mut AMediaExtractor,
        index: usize,
    ) -> *mut AMediaFormat;

    /// Since: API 21
    fn AMediaExtractor_selectTrack(extractor: *mut AMediaExtractor, index: usize) -> isize;

    /// Since: API 21
    fn AMediaExtractor_unselectTrack(extractor: *mut AMediaExtractor, index: usize) -> isize;

    /// Since: API 21
    fn AMediaExtractor_readSampleData(
        extractor: *mut AMediaExtractor,
        buffer: *mut u8,
        capacity: usize,
    ) -> isize;

    /// Since: API 21
    fn AMediaExtractor_getSampleFlags(extractor: *mut AMediaExtractor) -> u32;

    /// Since: API 21
    fn AMediaExtractor_getSampleTrackIndex(extractor: *mut AMediaExtractor) -> i32;

    /// Since: API 21
    fn AMediaExtractor_getSampleTime(extractor: *mut AMediaExtractor) -> i64;

    /// Since: API 21
    fn AMediaExtractor_advance(extractor: *mut AMediaExtractor) -> bool;
}

#[derive(Debug)]
pub struct MediaExtractor {
    inner: *mut AMediaExtractor,
    has_next: bool,
}

impl MediaExtractor {
    fn new() -> Self {
        Self {
            inner: unsafe { AMediaExtractor_new() },
            has_next: false,
        }
    }
    pub fn from_url(path: &str) -> Result<Self, MediaStatus> {
        unsafe {
            let mut me = Self::new();

            let path = CString::new(path).unwrap();

            let result = AMediaExtractor_setDataSource(me.inner, path.as_ptr());
            MediaStatus::make_result(result)?;

            me.has_next = true;

            Ok(me)
        }
    }

    pub fn track_count(&self) -> usize {
        unsafe { AMediaExtractor_getTrackCount(self.inner) }
    }

    pub fn track_index(&self) -> i32 {
        unsafe { AMediaExtractor_getSampleTrackIndex(self.inner) }
    }

    pub fn track_format(&self, index: usize) -> MediaFormat {
        unsafe {
            let fmt = AMediaExtractor_getTrackFormat(self.inner, index);
            MediaFormat::from_raw(fmt)
        }
    }

    pub fn select_track(&mut self, index: usize) {
        unsafe {
            AMediaExtractor_selectTrack(self.inner, index);
        }
    }

    pub fn unselect_track(&mut self, index: usize) {
        unsafe {
            AMediaExtractor_unselectTrack(self.inner, index);
        }
    }

    pub fn sample_flags(&self) -> u32 {
        unsafe { AMediaExtractor_getSampleFlags(self.inner) }
    }

    pub fn sample_time(&self) -> i64 {
        unsafe { AMediaExtractor_getSampleTime(self.inner) }
    }

    /// Read a packet into `buffer` and advance the extractor
    pub fn read_next(&mut self, buffer: &mut CodecInputBuffer) -> bool {
        unsafe {
            // debug!(
            //     "Writing to buffer {:p} with size: {}",
            //     buffer.buffer, buffer.size
            // );
            let count = AMediaExtractor_readSampleData(self.inner, buffer.buffer, buffer.size);

            // debug!("Write count: {count}, flags: {}", self.sample_flags());

            if count > 0 {
                buffer.set_write_size(count as usize);
                buffer.set_time(self.sample_time() as u64);
                buffer.set_flags(self.sample_flags());
                // TODO: Use the return value???
                self.has_next = AMediaExtractor_advance(self.inner);
                return true;
            }

            false
        }
    }

    pub fn has_next(&self) -> bool {
        self.has_next
    }
}

impl Drop for MediaExtractor {
    fn drop(&mut self) {
        unsafe {
            info!("Deleting the extractor...");
            AMediaExtractor_delete(self.inner);
        }
    }
}

unsafe impl Send for MediaExtractor {}
