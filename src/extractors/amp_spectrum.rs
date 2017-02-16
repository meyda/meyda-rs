extern crate rustfft;
extern crate num;

pub fn compute(signal : &Vec<f64>) -> Vec<f64> {
  let fft_len = signal.len();
  let mut fft = rustfft::FFT::new(fft_len, false);

  let complex_signal : Vec<_> = signal.iter()
    .map(|&sample| num::Complex::new(sample, 0_f64))
    .into_iter().collect();

  let mut spectrum = vec![num::Complex{re: 0.0, im: 0.0}; fft_len];

  fft.process(&complex_signal, &mut spectrum);

  let amp_spectrum = spectrum.iter()
    .map(|bin| {
      let tmp = bin.re.powf(2_f64) + bin.im.powf(2_f64);
      tmp.sqrt()
    })
    .into_iter().collect();

  return amp_spectrum;
}