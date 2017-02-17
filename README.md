# meyda-rs

*It's like [meyda](https://github.com/hughrawlinson/meyda), but for Rust.*

A side-project I'm using to learn Rust and get back to the realm of sensible, typed languages.

The plan is to initially provide a set of pure functions which operate on 64-bit vectors, each vector being a frame of audio.

Later on, I might look into file loading, config, overlapping frames, etc. etc. to approach the API of Meyda.

## Usage

This example creates a 1024-sample frame of white noise, and calculates its RMS and power spectrum.

```rust
extern crate meyda;
extern crate rand;

use rand::Rng;

fn main() {
  const BUFFER_SIZE: usize = 1024;

  // create a vector of white noise
  let mut generator = rand::thread_rng();
  let signal: Vec<f64> = vec![0; BUFFER_SIZE].iter()
    .map(|&sample| generator.gen_range(-1_f64, 1_f64))
    .collect();

  // compute features
  let rms = meyda::get_rms(&signal);
  let zcr = meyda::get_zcr(&signal);
  let power_spectrum = meyda::get_power_spectrum(&signal);

  println!("RMS is {} \n power spectrum is {:?}, zcr is {:?}", rms, power_spectrum, zcr);
}
```