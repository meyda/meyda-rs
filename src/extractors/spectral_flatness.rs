use extractors::amp_spectrum;

/**
 * @brief      SPECTRAL FLATNESS
 *
 * @param      signal  The signal vector (Vec::<f64>)
 *
 * @return     the spectral flatness value (f64)
 */
pub fn compute(signal: &Vec<f64>) -> f64 {
    let amp_spec: Vec<f64> = amp_spectrum::compute(signal);

    let fraction = amp_spec
        .iter()
        .fold((0_f64, 0_f64), |acc, &x| (acc.0 + x.log(10.0), x));

    (fraction.0 / amp_spec.len() as f64).exp() * amp_spec.len() as f64 / fraction.1
}
