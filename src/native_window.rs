use std::{ffi::c_void, ops::BitOr, ptr::null_mut};

use jni::{objects::JObject, JNIEnv};

/// Represents an image buffer (or a Surface in Java)
#[repr(C)]
#[derive(Debug)]
pub struct ANativeWindow {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

/// Window Formats
#[derive(Debug, Clone, Copy)]
pub enum NativeWindowFormat {
    /// 32 bits per pixel (8 bits per channel)
    ///
    /// GLES: GL_RGBA8
    Rgba8 = 1,
    /// 32 bits per pixel (8 bits per channel, alpha values are ignored)
    ///
    /// GLES: GL_RGB8
    Rgb8 = 2,
    /// 32 bits per pixel (8 bits per channel, alpha values are ignored)
    ///
    /// GLES: GL_RGB565
    Rgb565 = 4,

    /// YUV 420 888 format
    ///
    /// Must have an even width and height. Can be accessed in OpenGL shaders through an external sampler.
    /// Does not support mip-maps, cube-maps or multi-layered textures.
    Yuv420 = 0x23,
    /// Some unknown, undocumented format
    Other,
}

impl NativeWindowFormat {
    fn values() -> Vec<Self> {
        vec![Self::Rgba8, Self::Rgb8, Self::Rgb565, Self::Yuv420]
    }
}

impl From<isize> for NativeWindowFormat {
    fn from(value: isize) -> Self {
        let values = Self::values();

        for item in values {
            if item as isize == value {
                return item;
            }
        }

        Self::Other
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum NativeWindowTransform {
    Identity = 0x00,
    MirrorHorizontal = 0x01,
    MirrorVertical = 0x02,
    Rotate90 = 0x04,
    Rotate180 = NativeWindowTransform::MirrorHorizontal as isize
        | NativeWindowTransform::MirrorVertical as isize,
    Rotate270 =
        NativeWindowTransform::Rotate180 as isize | NativeWindowTransform::Rotate90 as isize,
}

impl BitOr for NativeWindowTransform {
    type Output = isize;

    fn bitor(self, rhs: Self) -> Self::Output {
        self as isize | rhs as isize
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct NativeWindowBuffer {
    pub width: i32,
    pub height: i32,
    pub stride: i32,
    pub format: i32,
    pub bits: *mut c_void,
    /// Do not touch!
    reserved: [u32; 6],
    window: *mut ANativeWindow,
}

impl NativeWindowBuffer {
    fn new(window: *mut ANativeWindow) -> Self {
        // Acquire a reference to this window so that it doesn't get dropped when we drop the parent
        unsafe { ANativeWindow_acquire(window) };
        Self {
            width: 0,
            height: 0,
            stride: 0,
            format: 0,
            reserved: [0; 6],
            bits: null_mut(),
            window,
        }
    }
}

impl Drop for NativeWindowBuffer {
    fn drop(&mut self) {
        if !self.window.is_null() {
            unsafe {
                ANativeWindow_unlockAndPost(self.window);
            }
            NativeWindow::from_raw(self.window);
        }
    }
}

#[repr(C)]
pub struct ARect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

// Functions start

#[link(name = "android")]
extern "C" {
    fn ANativeWindow_fromSurface(env: JNIEnv, surface: JObject) -> *mut ANativeWindow;

    #[cfg(feature = "api26")]
    /// Since API 26
    fn ANativeWindow_toSurface(env: JNIEnv, window: *mut ANativeWindow) -> JObject;

    fn ANativeWindow_acquire(window: *mut ANativeWindow);

    fn ANativeWindow_release(window: *mut ANativeWindow);

    fn ANativeWindow_getWidth(window: *mut ANativeWindow) -> i32;

    fn ANativeWindow_getHeight(window: *mut ANativeWindow) -> i32;

    fn ANativeWindow_getFormat(window: *mut ANativeWindow) -> i32;

    fn ANativeWindow_setBuffersGeometry(
        window: *mut ANativeWindow,
        width: i32,
        height: i32,
        format: i32,
    ) -> i32;

    fn ANativeWindow_lock(
        window: *mut ANativeWindow,
        buffer: *mut NativeWindowBuffer,
        rect: *mut ARect,
    ) -> i32;

    fn ANativeWindow_unlockAndPost(window: *mut ANativeWindow) -> i32;
}

// Functions end

#[derive(Debug)]
pub struct NativeWindow {
    pub(crate) inner: *mut ANativeWindow,
}

impl NativeWindow {
    pub fn from_raw(inner: *mut ANativeWindow) -> Self {
        Self { inner }
    }

    pub fn from_surface(surface: JObject) -> Self {
        unsafe {
            let env = javavm::get_env();
            Self::from_raw(ANativeWindow_fromSurface(env, surface))
        }
    }

    #[cfg(feature = "api26")]
    pub fn to_surface(&self) -> JObject {
        let env = javavm::get_env();
        unsafe { ANativeWindow_toSurface(env, self.inner) }
    }

    pub fn width(&self) -> i32 {
        unsafe { ANativeWindow_getWidth(self.inner) }
    }

    pub fn height(&self) -> i32 {
        unsafe { ANativeWindow_getHeight(self.inner) }
    }

    pub fn format(&self) -> NativeWindowFormat {
        unsafe { NativeWindowFormat::from(ANativeWindow_getFormat(self.inner) as isize) }
    }

    pub fn set_geometry(&mut self, width: i32, height: i32, format: NativeWindowFormat) {
        unsafe {
            ANativeWindow_setBuffersGeometry(self.inner, width, height, format as i32);
        }
    }

    /// Lock the window's next surface for writing. `bounds` is used as an in/out parameter, upon entering the function, it contains the dirty region, that is, the region the caller intends to redraw. When the function returns, `bounds` is updated with the actual area the caller needs to redraw
    ///
    /// Returns The `NativeWindowBuffer` on success, and None on error.
    ///
    /// The window's surface will be unlocked automatically when the buffer is dropped.
    pub fn lock(&mut self, bounds: &mut ARect) -> Option<NativeWindowBuffer> {
        unsafe {
            let mut buffer = NativeWindowBuffer::new(self.inner);
            if ANativeWindow_lock(self.inner, &mut buffer, bounds) == 0 {
                return Some(buffer);
            }
        }
        None
    }
}

impl Clone for NativeWindow {
    fn clone(&self) -> Self {
        unsafe {
            ANativeWindow_acquire(self.inner);
            Self { inner: self.inner }
        }
    }
}

impl Drop for NativeWindow {
    fn drop(&mut self) {
        unsafe {
            ANativeWindow_release(self.inner);
        }
    }
}
