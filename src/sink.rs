use fon::{chan::Channel, Sink, Frame};

use crate::{sample::ChanSampleFormat, source::MonoFrameSource};

pub struct PlaybackMonoSink<'a> {
    stream: &'a rodio::OutputStreamHandle,
    sample_rate: std::num::NonZeroU32,
    len: usize
}

impl<'a> PlaybackMonoSink<'a> {
    pub fn new(
        stream: &'a rodio::OutputStreamHandle,
        sample_rate: std::num::NonZeroU32,
        len: usize
    ) -> Self { PlaybackMonoSink { stream: stream, sample_rate: sample_rate, len: len } }
}
impl std::fmt::Debug for PlaybackMonoSink<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}


impl<'a, Chan: Channel + ChanSampleFormat> Sink<Chan, 1> for PlaybackMonoSink<'a> {
    fn sample_rate(&self) -> std::num::NonZeroU32 { self.sample_rate }
    fn len(&self) -> usize { self.len }

    fn sink_with(&mut self, iter: &mut dyn Iterator<Item = Frame<Chan, 1>>) {
        println!("rate: {}, len: {}", Sink::<Chan, 1>::sample_rate(self), Sink::<Chan, 1>::len(self));
        let sink = rodio::Sink::try_new(self.stream).unwrap();
        let source = MonoFrameSource::new(iter, Sink::<Chan, 1>::sample_rate(self).get(), Sink::<Chan, 1>::len(self)).collect_clone();
        sink.append(source);
        sink.sleep_until_end();
    }
}

