use fon::{chan::Channel, Frame};
use rodio::{Source, Sample};


use crate::sample::{MonoFrameSmaple, ChanSampleFormat};


pub(crate) struct IterSource<I: Iterator> {
    i: I,
    sample_rate: u32
}
 
impl<I: Iterator> IterSource<I> {
    pub fn new(i: I, sample_rate: u32) -> Self { Self { i: i, sample_rate: sample_rate } }
} 


impl<T, I: Iterator<Item = T>> Iterator for IterSource<I> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> { self.i.next() }
}


impl<T: Sample, I: Iterator<Item = T>> Source for IterSource<I> {
    fn current_frame_len(&self) -> Option<usize> { None }
    fn channels(&self) -> u16 { 1 }
    fn sample_rate(&self) -> u32 { self.sample_rate }
    fn total_duration(&self) -> Option<std::time::Duration> { None }
}

unsafe impl<T, I: Iterator<Item = T>> Send for IterSource<I> {}


pub(crate) struct MonoFrameSource<'a, Chan: Channel> {
    frame_iter: &'a mut dyn Iterator<Item = Frame<Chan, 1>>,
    sample_rate: u32,
    limit_count: usize
}

impl<'a, Chan: Channel + ChanSampleFormat> MonoFrameSource<'a, Chan> {
    pub fn new(frame_iter: &'a mut dyn Iterator<Item = Frame<Chan, 1>>, sample_rate: u32, limit_count: usize) -> Self { 
        MonoFrameSource { frame_iter: frame_iter, sample_rate: sample_rate, limit_count: limit_count } 
    }

    pub fn collect_clone(self) -> IterSource<<Vec<f32> as IntoIterator>::IntoIter> {
        let sr = (&self).sample_rate;
        IterSource { i: self.collect::<Vec<_>>().into_iter(), sample_rate: sr }
    }
}

impl<'a, Chan: Channel + ChanSampleFormat> Iterator for MonoFrameSource<'a, Chan> {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.limit_count == 0 {
            None
        } else {
            self.limit_count -= 1;
            self.frame_iter.next()
                .map(MonoFrameSmaple::from_frame)
                .map(|x| x.into_f32())
        }
    }
}

impl<'a, Chan: Channel + ChanSampleFormat> Source for MonoFrameSource<'a, Chan> {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 { 
        1 
    }

    fn sample_rate(&self) -> u32 { self.sample_rate }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

unsafe impl<'a, Chan: Channel + ChanSampleFormat> Send for MonoFrameSource<'a, Chan> {}
