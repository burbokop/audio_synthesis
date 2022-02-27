use fon::{chan::Channel, Sink, Frame};

use crate::{sample::ChanSampleFormat, source::MonoFrameSource};

pub struct PlaybackMonoSink<'a> {
    stream: &'a rodio::OutputStreamHandle
}

impl<'a> PlaybackMonoSink<'a> {
    pub fn new(stream: &'a rodio::OutputStreamHandle) -> Self { PlaybackMonoSink { stream: stream } }
}
impl std::fmt::Debug for PlaybackMonoSink<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}


impl<'a, Chan: Channel + ChanSampleFormat> Sink<Chan, 1> for PlaybackMonoSink<'a> {
    fn sample_rate(&self) -> std::num::NonZeroU32 { std::num::NonZeroU32::new(48_000).unwrap() }
    fn len(&self) -> usize { 48_000 * 10 }

    fn sink_with(&mut self, iter: &mut dyn Iterator<Item = Frame<Chan, 1>>) {
        println!("rate: {}, len: {}", Sink::<Chan, 1>::sample_rate(self), Sink::<Chan, 1>::len(self));
        let sink = rodio::Sink::try_new(self.stream).unwrap();
        let source = MonoFrameSource::new(iter, Sink::<Chan, 1>::sample_rate(self).get(), Sink::<Chan, 1>::len(self)).collect_clone();
        sink.append(source);
        sink.sleep_until_end();
        //self.stream.play_raw(mf).unwrap();
    }
}

