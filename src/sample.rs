use fon::chan::{Ch16, Ch32, Ch64};
use fon::{Frame, chan::Channel};
use rodio::{Sample};

use rodio::cpal::Sample as CpalSample;




#[derive(Clone, Copy)]
pub struct MonoFrameSmaple<Chan: Channel> {
    frame: Frame<Chan, 1>
}

impl<Chan: Channel> MonoFrameSmaple<Chan> {
    pub fn new(chan: Chan) -> Self { MonoFrameSmaple { frame: Frame::<Chan, 1>::new(chan) } }
    pub fn from_frame(frame: Frame<Chan, 1>) -> Self { MonoFrameSmaple { frame: frame } }
    pub fn into_chan(self) -> Chan { self.frame.channels()[0] }
    pub fn into_f32(self) -> f32 { self.frame.channels()[0].to_f32() }
}

//Ch16   i16
//Ch24   (i16 u8)
//Ch32   f32
//Ch64   f64

pub trait ChanSampleFormat {
    const FORMAT: rodio::cpal::SampleFormat;
}

impl ChanSampleFormat for Ch16 {
    const FORMAT: rodio::cpal::SampleFormat = rodio::cpal::SampleFormat::I16;
}

impl ChanSampleFormat for Ch32 {
    const FORMAT: rodio::cpal::SampleFormat = rodio::cpal::SampleFormat::F32;
}


unsafe impl<Chan: Channel + ChanSampleFormat> CpalSample for MonoFrameSmaple<Chan> {
    const FORMAT: rodio::cpal::SampleFormat = Chan::FORMAT;

    fn to_f32(&self) -> f32 { self.frame.channels()[0].to_f32() }

    fn to_i16(&self) -> i16 { unimplemented!() }
    fn to_u16(&self) -> u16 { unimplemented!() }

    fn from<S>(s: &S) -> Self where S: CpalSample {
        MonoFrameSmaple { frame: Frame::<Chan, 1>::new(Chan::from(s.to_f32())) }
    }
}



impl<Chan: Channel + ChanSampleFormat> Sample for MonoFrameSmaple<Chan> {
    fn lerp(first: Self, second: Self, numerator: u32, denominator: u32) -> Self {
        assert!(denominator == 0);
        Self::new(first.into_chan().lerp(second.into_chan(), Chan::from(numerator as f32 / denominator as f32)))
    }

    fn amplify(self, value: f32) -> Self { Self::new(self.into_chan() * Chan::from(value)) }
    fn saturating_add(self, other: Self) -> Self { Self::new(self.into_chan() + other.into_chan()) }
    fn zero_value() -> Self { Self::new(Chan::MID) }
}