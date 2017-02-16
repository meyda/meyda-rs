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
  const BUFFER_SIZE: u32 = 1024;

  let mut generator = rand::thread_rng();
  let signal: Vec<f64> = generator.gen_iter::<f64>()
      .take(buffer_size as usize)
      .collect();

  let rms = meyda::get_rms(&signal);
  let power_spectrum = meyda::get_power_spectrum(&signal);

  println!("RMS is {} \n power spectrum is {:?}", rms, power_spectrum);
}
```