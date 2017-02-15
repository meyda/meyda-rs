# meyda-rs

*It's like [meyda](https://github.com/hughrawlinson/meyda), but for Rust.*

A side-project I'm using to learn Rust and get back to the realm of sensible, typed languages.

## Usage

```rust
extern crate meyda;

fn main() {
  let signal = vec![0.2_f64, 0.5_f64, 0.7_f64];

  let rms = meyda::get_rms(&signal);

  println!("RMS is {}", rms);
}
```