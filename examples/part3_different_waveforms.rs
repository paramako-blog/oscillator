use std::f32::consts::TAU;

pub enum Waveform {
    Sine,
    Square,
    Saw,
    Triangle,
}

pub struct Oscillator {
    phase: f32,
    phase_increment: f32,
    sample_rate: f32,
    waveform: Waveform,
}

impl Oscillator {
    pub fn new(sample_rate: f32, frequency: f32, waveform: Waveform) -> Self {
        let phase_increment = frequency / sample_rate;
        Self {
            phase: 0.0,
            phase_increment,
            sample_rate,
            waveform,
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.phase_increment = frequency / self.sample_rate;
    }

    pub fn next_sample(&mut self) -> f32 {
        self.phase += self.phase_increment;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        match self.waveform {
            Waveform::Sine => (self.phase * TAU).sin(),
            Waveform::Square => {
                if self.phase < 0.5 {
                    1.0
                } else {
                    -1.0
                }
            }
            Waveform::Saw => 2.0 * self.phase - 1.0,
            Waveform::Triangle => {
                if self.phase < 0.5 {
                    4.0 * self.phase - 1.0
                } else {
                    -4.0 * self.phase + 3.0
                }
            }
        }
    }
}

fn main() {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer =
        hound::WavWriter::create("melody_struct.wav", spec).expect("Failed to create WAV file");

    let sample_rate = spec.sample_rate as f32;
    let duration_secs = 8.0;
    let amplitude = i16::MAX as f32;
    let total_samples = (sample_rate * duration_secs) as usize;

    let mut osc = Oscillator::new(sample_rate, 440.0, Waveform::Sine);

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
            _ => 880.0,    // A6
        };

        osc.set_frequency(freq_hz);

        let sample = (osc.next_sample() * amplitude) as i16;
        writer.write_sample(sample).unwrap();
    }

    writer.finalize().unwrap();
    println!("✅ Melody written to 'melody_struct.wav'");
}
