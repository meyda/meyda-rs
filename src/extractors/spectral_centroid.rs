use utils;
use extractors::amp_spectrum;

/**
 * @brief      SPECTRAL CENTROID
 *
 * @param      signal  The signal vector (Vec::<f64>)
 *
 * @return     the spectral centroid value (f64)
 */
pub fn compute(signal: &Vec<f64>) -> f64 {
    let amp_spec: Vec<f64> = amp_spectrum::compute(signal);

    utils::mu(1, &amp_spec)
}
