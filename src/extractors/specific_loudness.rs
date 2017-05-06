use utils;
use extractors::amp_spectrum as amp_spectrum;

/**
 * @brief      SPECIFIC BARK LOUDNESS
 *
 * @param      signal  The signal vector (Vec::<f64>)
 * @param      sample_rate  The signal sample rate (f64)
 *
 * @return     Loudness in each Bark band (Vec::<f64>)
 */
pub fn compute(signal: &Vec<f64>, sample_rate: f64) -> Vec<f64> {
  let mut amp_spec: Vec<f64> = amp_spectrum::compute(signal);

  let original_len = amp_spec.len();

  let bark_limits: Vec<usize> = utils::bark_limits(
    amp_spec.len(),
    sample_rate,
    amp_spec.len() as f64 / 2.0
  );

  let loudnesses = bark_limits[1..]
    .iter()
    .map(|&lim| {
      let current_limit = lim - (original_len - amp_spec.len());

      let end = match current_limit {
        e if amp_spec.len() >= e  => e,
        _                         => amp_spec.len(),
      };

      amp_spec.drain(0..end).sum::<f64>().powf(0.23)
    })
    .collect();

  loudnesses
}

