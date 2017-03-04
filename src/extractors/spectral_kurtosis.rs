use utils;
use extractors::amp_spectrum as amp_spectrum;

/**
 * @brief      SPECTRAL KURTOSIS
 *
 * @param      signal  The signal vector (Vec::<f64>)
 *
 * @return     the spectral kurtosis value (f64)
 */
pub fn compute(signal : &Vec<f64>) -> f64 {
  let amp_spec: Vec<f64> = amp_spectrum::compute(signal);

  let mus : Vec<f64> = vec![0; 4].iter()
    .enumerate()
    .map(|x| utils::mu(x.0 as i32, &amp_spec))
    .into_iter()
    .collect();

  let numerator = -3.0 * mus[0].powf(4.0) + 6.0 * mus[0] * mus[1] - 4.0 * mus[0] * mus[2] + mus[3];

  let denominator = (mus[1] - mus[0].powf(2.0)).sqrt().powf(4.0);

  numerator / denominator
}