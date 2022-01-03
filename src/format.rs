use std::{
    ffi::{c_void, CStr, CString},
    os::raw::c_char,
    ptr::null_mut,
};

use log::debug;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AMediaFormat {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[link(name = "mediandk")]
extern "C" {
    /// Available since API level 21.
    fn AMediaFormat_new() -> *mut AMediaFormat;

    /// Available since API level 21.
    fn AMediaFormat_delete(format: *mut AMediaFormat) -> isize;

    /// Available since API level 21.
    fn AMediaFormat_toString(format: *mut AMediaFormat) -> *const c_char;

    /// Available since API level 21.
    fn AMediaFormat_getInt32(format: *mut AMediaFormat, name: *const c_char, out: *mut i32)
        -> bool;

    /// Available since API level 21.
    fn AMediaFormat_getInt64(format: *mut AMediaFormat, name: *const c_char, out: *mut i64)
        -> bool;

    /// Available since API level 21.
    fn AMediaFormat_getFloat(format: *mut AMediaFormat, name: *const c_char, out: *mut f32)
        -> bool;

    /// Available since API level 28.
    #[cfg(feature = "api28")]
    fn AMediaFormat_getDouble(
        format: *mut AMediaFormat,
        name: *const c_char,
        out: *mut f64,
    ) -> bool;

    /// Available since API level 28.
    #[cfg(feature = "api28")]
    fn AMediaFormat_getRect(
        format: *mut AMediaFormat,
        name: *const c_char,
        left: *mut i32,
        top: *mut i32,
        right: *mut i32,
        bottom: *mut i32,
    ) -> bool;

    /// Available since API level 21.
    fn AMediaFormat_getSize(
        format: *mut AMediaFormat,
        name: *const c_char,
        out: *mut usize,
    ) -> bool;

    /// Available since API level 21.
    fn AMediaFormat_getBuffer(
        format: *mut AMediaFormat,
        name: *const c_char,
        out: *mut *mut c_void,
        size: *mut usize,
    ) -> bool;

    /// Available since API level 21.
    fn AMediaFormat_getString(
        format: *mut AMediaFormat,
        name: *const c_char,
        out: *mut *mut c_char,
    ) -> bool;

    /// Available since API level 21.
    fn AMediaFormat_setInt32(format: *mut AMediaFormat, name: *const c_char, value: i32) -> bool;

    /// Available since API level 21.
    fn AMediaFormat_setInt64(format: *mut AMediaFormat, name: *const c_char, value: i64) -> bool;

    /// Available since API level 21.
    fn AMediaFormat_setFloat(format: *mut AMediaFormat, name: *const c_char, value: f32) -> bool;

    /// Available since API level 28.
    #[cfg(feature = "api28")]
    fn AMediaFormat_setDouble(format: *mut AMediaFormat, name: *const c_char, value: f64) -> bool;

    /// Available since API level 28.
    #[cfg(feature = "api28")]
    fn AMediaFormat_setSize(format: *mut AMediaFormat, name: *const c_char, value: usize) -> bool;

    /// Available since API level 28.
    #[cfg(feature = "api28")]
    fn AMediaFormat_setRect(
        format: *mut AMediaFormat,
        name: *const c_char,
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
    ) -> bool;

    /// Available since API level 21.
    fn AMediaFormat_setString(
        format: *mut AMediaFormat,
        name: *const c_char,
        value: *const c_char,
    ) -> bool;

    /// Available since API level 21.
    fn AMediaFormat_setBuffer(
        format: *mut AMediaFormat,
        name: *const c_char,
        value: *const c_void,
        size: usize,
    ) -> bool;

    /// Available since API level 29.
    #[cfg(feature = "api29")]
    fn AMediaFormat_clear(format: *mut AMediaFormat);

    /// Available since API level 29.
    #[cfg(feature = "api29")]
    fn AMediaFormat_copy(to: *mut AMediaFormat, from: *mut AMediaFormat) -> isize;
}

/// This structure stores data in key-value pairs for use in MediaCodec and other places in the NDK
#[derive(Debug)]
pub struct MediaFormat {
    pub(crate) inner: *mut AMediaFormat,
}

impl MediaFormat {
    /// Construct a MediaFormat from a raw pointer
    pub fn from_raw(inner: *mut AMediaFormat) -> Self {
        Self { inner }
    }

    /// Create a new MediaFormat
    pub fn new() -> Option<Self> {
        unsafe {
            let inner = AMediaFormat_new();

            if inner.is_null() {
                return None;
            }

            Some(Self { inner })
        }
    }

    /// Set a 32-bit integer value
    pub fn set_i32(&mut self, name: &str, value: i32) -> bool {
        unsafe { AMediaFormat_setInt32(self.inner, name.as_ptr().cast(), value) }
    }

    /// Get a 32-bit integer value
    pub fn get_i32(&self, name: &str) -> Option<i32> {
        let mut value = None;

        unsafe {
            let mut v = 0;
            let name = CString::new(name).unwrap();
            if AMediaFormat_getInt32(self.inner, name.as_ptr(), &mut v) {
                value = Some(v);
            }
        }

        value
    }

    /// Set a 64-bit integer value
    pub fn set_i64(&mut self, name: &str, value: i64) -> bool {
        let name = CString::new(name).unwrap();
        unsafe { AMediaFormat_setInt64(self.inner, name.as_ptr(), value) }
    }

    /// Get a 64-bit integer value
    pub fn get_i64(&self, name: &str) -> Option<i64> {
        let mut value = None;

        unsafe {
            let mut v = 0;
            let name = CString::new(name).unwrap();
            if AMediaFormat_getInt64(self.inner, name.as_ptr(), &mut v) {
                value = Some(v);
            }
        }

        value
    }

    /// Set a 32-bit floating-point value
    pub fn set_f32(&mut self, name: &str, value: f32) -> bool {
        let name = CString::new(name).unwrap();
        unsafe { AMediaFormat_setFloat(self.inner, name.as_ptr(), value) }
    }

    /// Get a 32-bit floating-point value
    pub fn get_f32(&self, name: &str) -> Option<f32> {
        let mut value = None;

        unsafe {
            let mut v = 0f32;
            let name = CString::new(name).unwrap();
            if AMediaFormat_getFloat(self.inner, name.as_ptr(), &mut v) {
                value = Some(v);
            }
        }

        value
    }

    /// Convenience function to check whether the mime type is audio
    pub fn is_audio(&self) -> bool {
        if let Some(mime) = self.get_string("mime") {
            return mime.contains("audio");
        }

        false
    }

    /// Convenience function to check whether the mime type is video
    pub fn is_video(&self) -> bool {
        if let Some(mime) = self.get_string("mime") {
            return mime.contains("video");
        }

        false
    }

    /// Set a 64-bit floating point value
    #[cfg(feature = "api28")]
    pub fn set_f64(&mut self, name: &str, value: f64) -> bool {
        let name = CString::new(name).unwrap();
        unsafe { AMediaFormat_setDouble(self.inner, name.as_ptr(), value) }
    }

    /// Get a 64-bit floating-point value
    #[cfg(feature = "api28")]
    pub fn get_f64(&self, name: &str) -> Option<f64> {
        let value = None;

        unsafe {
            let mut v = 0f64;
            let name = CString::new(name).unwrap();
            if AMediaFormat_getDouble(self.inner, name.as_ptr(), &mut v) {
                value = Some(v);
            }
        }

        value
    }

    /// Set a string value
    pub fn set_string(&mut self, name: &str, value: &str) -> bool {
        let name = CString::new(name).unwrap();
        let value = CString::new(value).unwrap();
        unsafe { AMediaFormat_setString(self.inner, name.as_ptr(), value.as_ptr()) }
    }

    /// Get a string value
    pub fn get_string(&self, name: &str) -> Option<String> {
        let mut value = None;

        unsafe {
            let mut data = null_mut();
            let name = CString::new(name).unwrap();
            if AMediaFormat_getString(self.inner, name.as_ptr(), &mut data) {
                value = Some(CStr::from_ptr(data).to_string_lossy().to_string());
            }
        }

        value
    }

    /// Clear the entire buffer
    #[cfg(feature = "api29")]
    pub fn clear(&mut self) {
        unsafe {
            AMediaFormat_clear(self.inner);
        }
    }
}

impl ToString for MediaFormat {
    fn to_string(&self) -> String {
        unsafe {
            let value = AMediaFormat_toString(self.inner);
            if !value.is_null() {
                CStr::from_ptr(value).to_string_lossy().to_string()
            } else {
                String::new()
            }
        }
    }
}

impl Drop for MediaFormat {
    fn drop(&mut self) {
        unsafe {
            AMediaFormat_delete(self.inner);
        }
    }
}

unsafe impl Send for MediaFormat {}
unsafe impl Sync for MediaFormat {}
