use std::f32::consts::PI;

fn main() {
    // WAV file settings
    let spec = hound::WavSpec {
        channels: 1,        // mono
        sample_rate: 44100, // samples per second
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    // Create a WAV writer
    let mut writer = hound::WavWriter::create("sine.wav", spec).expect("Failed to create WAV file");
    // Sine wave parameters
    let freq_hz = 440.0; // frequency (A4)
    let duration_secs = 2.0; // 2 seconds
    let amplitude = i16::MAX as f32; // max volume

    let sample_rate = spec.sample_rate as f32;
    let total_samples = (sample_rate * duration_secs) as usize;

    for t in 0..total_samples {
        let time = t as f32 / sample_rate;
        let sample = (amplitude * (2.0 * PI * freq_hz * time).sin()) as i16;
        writer.write_sample(sample).unwrap();
    }

    writer.finalize().unwrap();
    println!("✅ Sine wave written to 'sine.wav'");
}
