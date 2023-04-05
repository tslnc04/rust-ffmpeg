use super::Context;
use ffi::*;
use libc::c_int;
use {
    util::format::{Pixel, Sample},
    ChannelLayout, Error, Frame, Rational,
};

pub struct Sink<'a> {
    ctx: &'a mut Context<'a>,
}

impl<'a> Sink<'a> {
    pub unsafe fn wrap<'b>(ctx: &'b mut Context<'b>) -> Sink<'b> {
        Sink { ctx }
    }

    pub fn width(&self) -> u32 {
        unsafe { av_buffersink_get_w(self.ctx.as_ptr()) as u32 }
    }

    pub fn height(&self) -> u32 {
        unsafe { av_buffersink_get_h(self.ctx.as_ptr()) as u32 }
    }

    pub fn time_base(&self) -> Rational {
        unsafe { Rational::from(av_buffersink_get_time_base(self.ctx.as_ptr())) }
    }

    pub fn frame_rate(&self) -> Rational {
        unsafe { Rational::from(av_buffersink_get_frame_rate(self.ctx.as_ptr())) }
    }

    pub fn pixel_format(&self) -> Result<Pixel, Error> {
        unsafe {
            if av_buffersink_get_type(self.ctx.as_ptr()) != AVMediaType::AVMEDIA_TYPE_VIDEO {
                return Err(Error::InvalidData);
            }

            // pretty dangerous but this should be same enough since it emulates
            // what is done in ffmpeg and we make sure it's a video sink
            let pixel_format =
                &av_buffersink_get_format(self.ctx.as_ptr()) as *const i32 as *const AVPixelFormat;
            Ok((*pixel_format).into())
        }
    }

    pub fn sample_format(&self) -> Result<Sample, Error> {
        unsafe {
            if av_buffersink_get_type(self.ctx.as_ptr()) != AVMediaType::AVMEDIA_TYPE_AUDIO {
                return Err(Error::InvalidData);
            }

            // same as above, could be dangerous but should be fine
            let sample_format =
                &av_buffersink_get_format(self.ctx.as_ptr()) as *const i32 as *const AVSampleFormat;
            Ok((*sample_format).into())
        }
    }

    pub fn sample_rate(&self) -> i32 {
        unsafe { av_buffersink_get_sample_rate(self.ctx.as_ptr()) }
    }

    pub fn channels(&self) -> i32 {
        unsafe { av_buffersink_get_channels(self.ctx.as_ptr()) }
    }

    // TODO(tslnc04): figure out how to convert this to the ffmpeg_rs type
    pub fn channel_layout(&self) -> ChannelLayout {
        unsafe {
            ChannelLayout::from_bits_truncate(av_buffersink_get_channel_layout(self.ctx.as_ptr()))
        }
    }
}

impl<'a> Sink<'a> {
    pub fn frame(&mut self, frame: &mut Frame) -> Result<(), Error> {
        unsafe {
            match av_buffersink_get_frame(self.ctx.as_mut_ptr(), frame.as_mut_ptr()) {
                n if n >= 0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn samples(&mut self, frame: &mut Frame, samples: usize) -> Result<(), Error> {
        unsafe {
            match av_buffersink_get_samples(
                self.ctx.as_mut_ptr(),
                frame.as_mut_ptr(),
                samples as c_int,
            ) {
                n if n >= 0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn set_frame_size(&mut self, value: u32) {
        unsafe {
            av_buffersink_set_frame_size(self.ctx.as_mut_ptr(), value);
        }
    }
}
