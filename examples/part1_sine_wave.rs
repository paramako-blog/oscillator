use std::f32::consts::PI;

fn main() {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create("sine.wav", spec).expect("Failed to create WAV file");

    let duration_secs = 4.0;
    let frequency = 440.0; // A4
    let amplitude = i16::MAX as f32;

    let sample_rate = spec.sample_rate as f32;
    let total_samples = (sample_rate * duration_secs) as usize;

    for t in 0..total_samples {
        let time = t as f32 / sample_rate;
        let sample = (amplitude * (2.0 * PI * frequency * time).sin()) as i16;
        writer.write_sample(sample).unwrap();
    }

    writer.finalize().unwrap();
    println!("Sine wave written to 'sine.wav'");
}
