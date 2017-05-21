# meyda-rs

[![Build Status](https://travis-ci.org/meyda/meyda-rs.svg?branch=master)](https://travis-ci.org/meyda/meyda-rs)

*It's like [meyda](https://github.com/hughrawlinson/meyda), but for Rust.*

This project is heavily WIP and it's not wise to use it in production yet.

The plan is to initially provide a set of pure functions which operate on 64-bit float vectors, each vector being a frame of audio.

Later on, `meyda-rs` should support file loading, configuration, overlapping frames, etc. to approach the API of Meyda.

## Usage

This example creates a 1024-sample frame of white noise, and calculates its features.

```rust
extern crate meyda;
extern crate rand;

use rand::Rng;

fn main() {
  const BUFFER_SIZE: usize = 1024;
  const SAMPLE_RATE: f64 = 44100.0;

  // create a vector of white noise
  let mut generator = rand::thread_rng();
  let signal: Vec<f64> = vec![0; BUFFER_SIZE].iter()
    .map(|&sample| generator.gen_range(-1_f64, 1_f64))
    .collect();

  // compute features
  let rms = meyda::get_rms(&signal);
  let energy = meyda::get_energy(&signal);
  let zcr = meyda::get_zcr(&signal);
  let power_spectrum = meyda::get_power_spectrum(&signal);
  let spectral_centroid = meyda::get_spectral_centroid(&signal);
  let spectral_flatness = meyda::get_spectral_flatness(&signal);
  let spectral_kurtosis = meyda::get_spectral_kurtosis(&signal);
  let spectral_rolloff = meyda::get_spectral_rolloff(&signal, SAMPLE_RATE, 0.95);
  let bark_loudness = meyda::get_bark_loudness(&signal, SAMPLE_RATE);

  println!("RMS is {} \n energy is {:?}, zcr is {:?},\n spectral centroid is {},\n spectral flatness is {},\n spectral kurtosis is {},\n spectral rolloff is {},\n Bark loudness is {}", rms, energy, zcr, spectral_centroid, spectral_flatness, spectral_kurtosis,
   spectral_rolloff, bark_loudness);
}
```

## Development

Contributions are always welcome. The library *should* be test-driven and all new features *should* have accompanying tests.

Tests can be run with `cargo test` – each extractor function *should* have a test module in the same file, and should make use of [meyda/gauge](https://github.com/meyda/gauge), which is submodule'd in `/src/utils`. The deserialized gauge data is provided by `utils::test` as a vector of `TestDataSet` structures.
