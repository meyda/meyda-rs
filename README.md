# meyda-rs

*It's like [meyda](https://github.com/hughrawlinson/meyda), but for Rust.*

A side-project I'm using to learn Rust and get back to the realm of sensible, typed languages.

The plan is to initially provide a set of pure functions which operate on 64-bit vectors, each vector being a frame of audio.

Later on, I might look into file loading, config, overlapping frames, etc. etc. to approach the API of Meyda.

## Usage

```rust
extern crate meyda;

fn main() {
  let signal = vec![0.2_f64, 0.5_f64, 0.7_f64];

  let rms = meyda::get_rms(&signal);
  let pow_spec = meyda::get_power_spectrum(&signal);

  println!("RMS is {}", rms);
}
```