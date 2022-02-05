use crate::extractors::amp_spectrum;

pub fn compute(signal: &Vec<f64>, sample_rate: f64, rolloff_point: Option<f64>) -> f64 {
    let amp_spec: Vec<f64> = amp_spectrum::compute(signal);

    let nyq_bin = sample_rate / (2.0 * (amp_spec.len() as f64 - 1.0));
    let mut integral: f64 = amp_spec.iter().sum();

    let threshold = match rolloff_point {
        Some(rf) => rf * integral,
        None => 0.99 * integral,
    };

    let mut reader = amp_spec.len() as i32 - 1;

    while integral > threshold && reader >= 0 {
        integral -= amp_spec[reader as usize];
        reader -= 1;
    }

    (reader + 1) as f64 * nyq_bin
}

#[cfg(test)]
mod tests {
    use super::compute;
    use crate::utils::test;
    use std::f64;

    const FLOAT_PRECISION: f64 = 0.000_000_010;

    fn test_against(dataset: &test::data::TestDataSet) -> () {
        let sr = compute(&dataset.signal, dataset.info.sampleRate as f64, None);

        assert_relative_eq!(
            sr,
            dataset.features.spectralRolloff,
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
