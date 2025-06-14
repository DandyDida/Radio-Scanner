# 📻 Radio Scanner (RTL-SDR) in Rust........
![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)

Radio Scanner adalah proyek eksperimental yang ditulis dengan bahasa pemrograman **Rust**, bertujuan untuk menangkap dan memproses sinyal radio menggunakan perangkat **RTL-SDR (Realtek Software Defined Radio)** berbasis chip **RTL2832U**.

---

## 🛠️ Teknologi yang Digunakan

- **Bahasa Pemrograman:** Rust 🦀
- **Perangkat Keras:** RTL-SDR USB Dongle (Realtek RTL2832U)
- **Crate Rust (Direncanakan / Digunakan):**
  - [`rtlsdr`](https://crates.io/crates/rtlsdr) – Interaksi dengan perangkat RTL-SDR
  - [`hound`](https://crates.io/crates/hound) – Penulisan file WAV
  - [`dasp`](https://crates.io/crates/dasp) – Pengolahan sinyal digital (opsional untuk DSP)
- **Antarmuka USB:** Menggunakan protokol USB untuk komunikasi antara dongle dan komputer

---

## 🧪 Cara Kerja (Teknis Singkat)

1. Dongle RTL-SDR menangkap sinyal radio analog dari udara.
2. Sinyal ini dikonversi ke bentuk digital melalui chip RTL2832U.
3. Data digital dikirimkan ke komputer melalui USB.
4. Program ini memproses sinyal digital tersebut (misalnya, untuk decoding AM/FM, visualisasi spektrum, atau perekaman suara).

---

## 🚧 Status Proyek

> **MASIH DALAM PENGEMBANGAN**  
> Beberapa fitur dasar sedang dalam tahap eksperimen. Modul decoding dan analisis sinyal sedang dikembangkan.

---

## 🎯 Tujuan Proyek

- Memberikan pemahaman mendalam tentang cara kerja Software Defined Radio (SDR)
- Menjadi eksperimen pembelajaran di bidang **radio frequency (RF)**, **Internet of Things (IoT)**, dan **sinyal digital**
- Menjadi pondasi bagi pengembangan tools seperti:
  - Spectrum analyzer
  - FM/AM audio decoder
  - Signal intelligence (SIGINT) untuk edukasi

---

## 📦 Instalasi

> Prasyarat:  
> - Rust sudah terinstal → [`https://rustup.rs`](https://rustup.rs)  
> - Dongle RTL-SDR sudah terhubung ke USB

```bash
git clone https://github.com/DandyDida/Radio-Scanner.git
cd Radio-Scanner
cargo build
