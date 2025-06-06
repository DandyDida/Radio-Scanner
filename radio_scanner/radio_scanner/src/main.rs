use rtlsdr::{RTLSDRDevice, TunerGain};
use hound::{WavWriter, WavSpec, SampleFormat};
use std::env;

/// Fungsi sederhana untuk demodulasi FM dari data I/Q
fn fm_demodulate(iq: &[u8]) -> Vec<i16> {
    let mut audio = Vec::with_capacity(iq.len() / 2);
    let mut last_phase = 0.0f32;
    for chunk in iq.chunks(2) {
        if chunk.len() < 2 { continue; }
        // Ubah ke [-1.0, 1.0]
        let i = (chunk[0] as f32 - 127.5) / 127.5;
        let q = (chunk[1] as f32 - 127.5) / 127.5;
        let phase = q.atan2(i);
        let diff = phase - last_phase;
        last_phase = phase;
        // Skala ke i16 audio
        audio.push((diff * 10_000.0) as i16);
    }
    audio
}

fn main() {
    // Baca frekuensi dari argumen, default 100 MHz
    let args: Vec<String> = env::args().collect();
    let freq: u32 = if args.len() > 1 {
        args[1].parse().unwrap_or(100_000_000)
    } else {
        100_000_000
    };

    let mut dev = rtlsdr::open(0).expect("Device not found");
    dev.set_center_freq(freq).expect("Gagal set frekuensi");
    dev.set_sample_rate(250_000).expect("Gagal set sample rate");
    dev.set_tuner_gain_mode(false).unwrap();
    dev.set_tuner_gain(TunerGain::Manual(0)).unwrap();

    let mut buffer = [0u8; 16384];

    let spec = WavSpec {
        channels: 1,
        sample_rate: 48000,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };
    let mut writer = WavWriter::create("output.wav", spec).unwrap();

    println!("Menangkap sinyal FM di {} Hz… Ctrl+C untuk berhenti", freq);

    for _ in 0..200 {
        dev.read_sync(&mut buffer).unwrap();
        let audio = fm_demodulate(&buffer);
        for sample in audio.iter().step_by(5) { // Downsample dari 250k ke ~50k
            writer.write_sample(*sample).unwrap();
        }
    }

    writer.finalize().unwrap();
    println!("Selesai. File output.wav siap didengarkan.");
}