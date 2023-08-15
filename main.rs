extern crate hound;
extern crate rand;

use hound::{WavSpec, WavWriter};
use rand::Rng;

fn main() {
    // Parameters
    let duration = 5; // Duration in seconds
    let sample_rate = 44100; // Sample rate in Hz
    let num_samples = duration * sample_rate;

    // Create a WavSpec
    let spec = WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    // Generate random audio data
    let mut rng = rand::thread_rng();
    let random_audio: Vec<i16> = (0..num_samples)
        .map(|_| rng.gen_range(i16::MIN..i16::MAX))
        .collect();

    // Create and write the WAV file
    let mut writer = WavWriter::create("random_audio.wav", spec).unwrap();
    for sample in random_audio {
        writer.write_sample(sample).unwrap();
    }

    println!("Random audio saved to random_audio.wav");
}
