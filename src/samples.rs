/// Represents a codec frame (either audio or video)
pub enum Frame<'a> {
    Audio(AudioFrame<'a>),
    Video(VideoFrame<'a>),
}

pub const ENCODING_PCM_16BIT: usize = 2;
pub const ENCODING_PCM_FLOAT: usize = 4;

/// Represents an audio sample format, and contains the samples buffer
pub enum SampleFormat<'a> {
    S16(&'a [i16]),
    F32(&'a [f32]),
}

impl SampleFormat<'_> {
    /// Returns the number of samples contained by this format
    fn samples(&self, channels: u32) -> usize {
        match self {
            SampleFormat::S16(value) => value.len() / channels as usize,
            SampleFormat::F32(value) => value.len() / channels as usize,
        }
    }

    /// Returns the size of one sample represented by this format
    fn sample_size(&self) -> usize {
        match self {
            SampleFormat::S16(_) => std::mem::size_of::<i16>(),
            SampleFormat::F32(_) => std::mem::size_of::<f32>(),
        }
    }

    /// Returns the size of one frame represented by this format. It needs the number of channels stored in this buffer to determine the value
    fn frame_size(&self, channels: u32) -> usize {
        self.sample_size() * channels as usize
    }
}

/// Represents an audio frame, with sample format and channels
pub struct AudioFrame<'a> {
    format: SampleFormat<'a>,
    channels: u32,
}

impl<'a> AudioFrame<'a> {
    /// Create the audio frame
    pub fn new(format: SampleFormat<'a>, channels: u32) -> Self {
        Self { format, channels }
    }

    /// Returns the number of channels for this frame
    pub fn channels(&self) -> u32 {
        self.channels
    }

    /// Returns the sample format for this frame
    pub fn format(&self) -> &SampleFormat {
        &self.format
    }

    /// Returns the number of samples contained in this frame
    pub fn nb_samples(&self) -> usize {
        self.format.samples(self.channels)
    }
}

pub enum VideoFrame<'a> {
    /// Represens a hardware video frame (stored in a NativeWindow, so it cannot be accessed)
    ///
    /// Can't do much with this, it's just a marker. The underlying `CodecOutputBuffer` will take care of it
    Hardware,
    /// Represents a raw video frame, with a specific pixel format and a buffer to read the data
    RawFrame(RawVideoFrame<'a>),
}

/// A raw video frame with pixel format and a byte buffer to read the data
pub struct RawVideoFrame<'a> {
    buffer: &'a [u8],
}
