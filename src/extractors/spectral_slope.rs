use extractors::amp_spectrum;

pub fn compute(signal: &Vec<f64>, sample_rate: f64) -> f64 {
    let amp_spec: Vec<f64> = amp_spectrum::compute(signal);

    let (amp_sum, freq_sum, pow_freq_sum, amp_freq_sum) = amp_spec.iter().enumerate().fold(
        (0.0, 0.0, 0.0, 0.0),
        |(amp_sum, freq_sum, pow_freq_sum, amp_freq_sum), (i, amp)| {
            let freq = (i as f64 * sample_rate) / amp_spec.len() as f64;

            (
                amp_sum + amp,
                freq_sum + freq,
                pow_freq_sum + freq.powf(2.0),
                amp_freq_sum + amp * freq,
            )
        },
    );
    (amp_spec.len() as f64 * amp_freq_sum - freq_sum * amp_sum) / amp_sum
        * (pow_freq_sum - freq_sum.powf(2.0))
}

#[cfg(test)]
mod tests {
    use super::compute;
    use std::f64;
    use utils::test;

    const FLOAT_PRECISION: f64 = 0.000_000_010;

    fn test_against(dataset: &test::data::TestDataSet) -> () {
        let spectral_slope_value = compute(&dataset.signal, dataset.info.sampleRate as f64);

        assert_relative_eq!(
            spectral_slope_value,
            dataset.features.spectralSlope,
            epsilon = f64::EPSILON,
            max_relative = FLOAT_PRECISION
        );
    }

    #[test]
    fn test_spectral_rolloff() {
        let datasets = test::data::get_all();

        for dataset in datasets.iter() {
            test_against(dataset);
        }
    }
}
