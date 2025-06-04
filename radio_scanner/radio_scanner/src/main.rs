use rtlsdr::{RTLSDRDevice, TunerGain};
use hound::{WavWriter, WavSpec, SampleFormat};
use std::fs::File;

fn main() {
    let mut dev = rtlsdr::open(0).expect("Device not found");
    dev.set_tuner_gain_mode(false).unwrap();
    dev.set_tuner_gain(TunerGain::Manual(0)).unwrap();

    // Buffer
    let mut buffer = [0u8; 16384];

    // Siapkan WAV Writer
    let spec = WavSpec {
        channels: 1,
        sample_rate: 48000,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };
    let mut writer = WavWriter::create("output.wav", spec).unwrap();

    println!("Mulai menangkap sinyal… Ctrl+C untuk berhenti");

    for _ in 0..100 {
        dev.read_sync(&mut buffer).unwrap();

        // Konversi I/O ke mono_audio dummy
        for chunk in buffer.chunks(2) {
            let sample = i16::from_le_bytes([chunk[0], chunk[1]]);
            writer.write_sample(sample).unwrap();
        }
    }
}
