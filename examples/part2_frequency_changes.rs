use std::f32::consts::PI;

fn main() {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer =
        hound::WavWriter::create("melody_fixed.wav", spec).expect("Failed to create WAV file");

    let duration_secs = 8.0;
    let amplitude = i16::MAX as f32;

    let sample_rate = spec.sample_rate as f32;
    let total_samples = (sample_rate * duration_secs) as usize;

    let mut phase = 0.0;

    for t in 0..total_samples {
        let time = t as f32 / sample_rate;

        let freq_hz = match time.ceil() {
            1.0 => 440.0,  // A4
            2.0 => 494.0,  // B4
            3.0 => 523.25, // C5
            4.0 => 587.33, // D5
            5.0 => 659.25, // E5
            6.0 => 698.46, // F5
            7.0 => 783.99, // G5
            _ => 880.0,    // A5
        };

        let phase_increment = 2.0 * PI * freq_hz / sample_rate;
        phase = (phase + phase_increment) % (2.0 * PI);

        let sample = (amplitude * phase.sin()) as i16;
        writer.write_sample(sample).unwrap();
    }

    writer.finalize().unwrap();
    println!("Melody written to 'melody_fixed.wav'");
}
