use fon::{chan::Channel, Frame};
use rodio::{Source, Sample};


use crate::sample::{MonoFrameSmaple, ChanSampleFormat};


pub(crate) struct IterSource<I: Iterator> {
    i: I
}

impl<T, I: Iterator<Item = T>> Iterator for IterSource<I> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> { self.i.next() }
}

impl<T: Sample, I: Iterator<Item = T>> Source for IterSource<I> {
    fn current_frame_len(&self) -> Option<usize> { None }
    fn channels(&self) -> u16 { 1 }
    fn sample_rate(&self) -> u32 { 48_000 }
    fn total_duration(&self) -> Option<std::time::Duration> { None }
}

unsafe impl<T, I: Iterator<Item = T>> Send for IterSource<I> {}



pub(crate) struct MonoFrameSource<'a, Chan: Channel> {
    frame_iter: &'a mut dyn Iterator<Item = Frame<Chan, 1>>
}

impl<'a, Chan: Channel + ChanSampleFormat> MonoFrameSource<'a, Chan> {
    pub fn new(frame_iter: &'a mut dyn Iterator<Item = Frame<Chan, 1>>) -> Self { MonoFrameSource { frame_iter: frame_iter } }

    pub fn collect_clone(self) -> IterSource<<Vec<f32> as IntoIterator>::IntoIter> {
        IterSource { i: self.collect::<Vec<_>>().into_iter() }
    }
}

impl<'a, Chan: Channel + ChanSampleFormat> Iterator for MonoFrameSource<'a, Chan> {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> { 
        self.frame_iter.next()
            .map(MonoFrameSmaple::from_frame)
            .map(|x| x.into_f32())
    }
}

impl<'a, Chan: Channel + ChanSampleFormat> Source for MonoFrameSource<'a, Chan> {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 { 
        1 
    }

    fn sample_rate(&self) -> u32 {
        48_000
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

unsafe impl<'a, Chan: Channel + ChanSampleFormat> Send for MonoFrameSource<'a, Chan> {}
