pub enum Samples<'a> {
    Audio(AudioSamples<'a>),
    Video(VideoSamples<'a>),
}

pub enum SampleFormat<'a> {
    S16(&'a [i16]),
    F32(&'a [f32]),
}

impl SampleFormat<'_> {
    fn sizeof(&self, channels: u32) -> usize {
        match self {
            SampleFormat::S16(value) => value.len() / channels as usize,
            SampleFormat::F32(value) => value.len() / channels as usize,
        }
    }
}

pub struct AudioSamples<'a> {
    format: SampleFormat<'a>,
    channels: u32,
}

impl<'a> AudioSamples<'a> {
    pub fn from_ptr(format: SampleFormat<'a>, channels: u32) -> Self {
        unsafe { Self { format, channels } }
    }

    pub fn channels(&self) -> u32 {
        self.channels
    }

    pub fn format(&self) -> &SampleFormat {
        &self.format
    }

    pub fn nb_samples(&self) -> usize {
        self.format.sizeof(self.channels)
    }
}

pub enum VideoSamples<'a> {
    Hardware,
    RawSamples(RawVideoSamples<'a>),
}

pub struct RawVideoSamples<'a> {
    buffer: &'a [u8],
}
