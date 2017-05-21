use extractors::amp_spectrum;
/**
 * @brief      POWER SPECTRUM
 *
 * @param      signal  The signal vector (Vec::<f64>)
 *
 * @return     The power spectrum vector (Vec::<f64>)
 */
pub fn compute(signal: &Vec<f64>) -> Vec<f64> {
    let amp_spec: Vec<f64> = amp_spectrum::compute(signal);

    let pow_spec: Vec<f64> = amp_spec.iter().map(|bin| bin.powi(2)).into_iter().collect();

    return pow_spec;
}
