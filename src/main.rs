use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use rodio::{Decoder, OutputStream};
use rodio::source::{SineWave, Source};








use fon::chan::{Ch16, Channel};
use fon::{Audio, Frame, Sink};
use twang::noise::White;
use twang::ops::Gain;
use twang::osc::Sine;
use twang::Synth;

/// First ten harmonic volumes of a piano sample (sounds like electric piano).
const HARMONICS: [f32; 10] = [
    0.700, 0.243, 0.229, 0.095, 0.139, 0.087, 0.288, 0.199, 0.124, 0.090,
];
/// The three pitches in a perfectly tuned A3 minor chord
const PITCHES: [f32; 3] = [220.0, 220.0 * 32.0 / 27.0, 220.0 * 3.0 / 2.0];
/// Volume of the piano
const VOLUME: f32 = 1.0 / 3.0;

// State of the synthesizer.
#[derive(Default)]
struct Processors {
    // White noise generator.
    white: White,
    // 10 harmonics for 3 pitches.
    piano: [[Sine; 10]; 3],
}

struct MySink<'a> {
    stream: &'a rodio::OutputStreamHandle
}

impl<'a> MySink<'a> {
    fn new(stream: &'a rodio::OutputStreamHandle) -> Self { MySink { stream: stream } }
}
impl std::fmt::Debug for MySink<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}


impl<Chan: Channel, const CH: usize> Sink<Chan, CH> for MySink<'_> {
    fn sample_rate(&self) -> std::num::NonZeroU32 { std::num::NonZeroU32::new(48_000).unwrap() }

    fn len(&self) -> usize { 48_000 * 10 }

    fn sink_with(&mut self, iter: &mut dyn Iterator<Item = Frame<Chan, CH>>) {
        self.stream.play_raw(iter);
        println!("sink_with");
        iter.for_each(|f| {
            println!("frame: {:?}", f);
        })
    }
}



fn main() {

    // Create audio processors
    let mut proc = Processors::default();
    // Adjust phases of harmonics.
    for pitch in proc.piano.iter_mut() {
        for harmonic in pitch.iter_mut() {
            harmonic.shift(proc.white.step());
        }
    }
    // Build synthesis algorithm
    let mut synth = Synth::new(proc, |proc, mut frame: Frame<_, 2>| {
        for (s, pitch) in proc.piano.iter_mut().zip(PITCHES.iter()) {
            for ((i, o), v) in s.iter_mut().enumerate().zip(HARMONICS.iter()) {
                // Get next sample from oscillator.
                let sample = o.step(pitch * (i + 1) as f32);
                // Pan the generated harmonic center
                frame = frame.pan(Gain.step(sample, (v * VOLUME).into()), 0.0);
            }
        }
        frame
    });


    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    //let sink = Sink::try_new(&stream_handle).unwrap();

    // Add a dummy source of the sake of the example.
    //let source = SineWave::new(440.0).take_duration(Duration::from_secs_f32(10.)).amplify(0.20);
    //sink.append(source);

    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.

    let sink = MySink::new(&stream_handle);
    let mut audio = Audio::<Ch16, 2>::with_silence(48_000, 48_000 * 5);
   
    //audio.sink()

    // Synthesize 5 seconds of audio
    println!("stream start");
    synth.stream::<Ch16, _>(sink);
    println!("stream end");

    
    //sink.sleep_until_end();
}