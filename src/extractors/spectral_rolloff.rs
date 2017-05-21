use extractors::amp_spectrum;

/**
 * @brief      SPECTRAL ROLLOFF
 *
 * @param      signal  The signal vector (Vec::<f64>)
 *
 * @return     the spectral rolloff value (f64)
 */
pub fn compute(signal: &Vec<f64>, sample_rate: f64, rolloff_point: Option<f64>) -> f64 {
    let amp_spec: Vec<f64> = amp_spectrum::compute(signal);

    let nyq_bin = sample_rate / (2.0 * amp_spec.len() as f64 - 1.0);
    let mut integral: f64 = amp_spec.iter().sum();

    let threshold = match rolloff_point {
        Some(rf) => rf * integral,
        None => 99.0 * integral,
    };

    let mut reader = amp_spec.len() as i32 - 1;

    while integral > threshold && reader >= 0 {
        integral -= amp_spec[reader as usize];
        reader -= 1;
    }

    (reader + 1) as f64 * nyq_bin
}
