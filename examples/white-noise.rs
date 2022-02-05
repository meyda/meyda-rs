extern crate meyda;
extern crate rand;

use rand::Rng;

fn main() {
  const BUFFER_SIZE: usize = 1024;
  const SAMPLE_RATE: f64 = 44100.0;

  // create a vector of white noise
  let mut generator = rand::thread_rng();
  let signal: Vec<f64> = vec![0; BUFFER_SIZE]
    .iter()
    .map(|&_sample| generator.gen_range(-1_f64..1_f64))
    .collect();

  // compute features
  let rms = meyda::get_rms(&signal);
  let energy = meyda::get_energy(&signal);
  let zcr = meyda::get_zcr(&signal);
  // let power_spectrum = meyda::get_power_spectrum(&signal);
  let spectral_centroid = meyda::get_spectral_centroid(&signal);
  let spectral_flatness = meyda::get_spectral_flatness(&signal);
  let spectral_kurtosis = meyda::get_spectral_kurtosis(&signal);
  let spectral_rolloff = meyda::get_spectral_rolloff(&signal, SAMPLE_RATE, Some(0.95));
  let bark_loudness = meyda::get_bark_loudness(&signal, SAMPLE_RATE);

  println!("RMS is {} \n energy is {:?}, zcr is {:?},\n spectral centroid is {},\n spectral flatness is {},\n spectral kurtosis is {},\n spectral rolloff is {},\n Bark loudness is {:?}", rms, energy, zcr, spectral_centroid, spectral_flatness, spectral_kurtosis,
   spectral_rolloff, bark_loudness);
}
