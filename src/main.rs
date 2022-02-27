mod source;
mod sample;
mod sink;
mod processors;
mod monitor;

use monitor::monitor_once;
use processors::{DeterioratorExt, IntegratorExt, AdderExt, AmplifierExt};
use rodio::OutputStream;

use fon::chan::Ch16;
use fon::Frame;
use source::IterSource;
use twang::noise::White;
use twang::ops::Gain;
use twang::osc::Sine;
use twang::Synth;

use crate::processors::sin_generator;
use crate::sink::PlaybackMonoSink;


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
    let mut synth = Synth::new(proc, |proc, mut frame: Frame<_, 1>| {

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



    let mut f = sin_generator(48000 * 10, 0.1).amplified(0.5); // deteriorated(4.);
    let mut f2 = sin_generator(48000 * 10, 0.025).amplified(0.25); // deteriorated(4.);

    let mut f3 = f.add(f2);

    

    monitor_once(&mut f3, 48000);

    let source = IterSource::new(f3, 48000);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let sink = rodio::Sink::try_new(&stream_handle).unwrap();
    sink.append(source);
    sink.sleep_until_end();


    //let sink = PlaybackMonoSink::new(&stream_handle, std::num::NonZeroU32::new(48_000).unwrap(), 48_000 * 10);
    //println!("stream start");
    //synth.stream::<Ch16, _>(sink);
    //println!("stream end");
}