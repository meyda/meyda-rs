/**
 * @brief      ROOT MEAN SQUARE
 *
 * @param      signal  The signal vector (Vec::<f64>)
 *
 * @return     the RMS value (f64)
 */
pub fn compute(signal: &Vec<f64>) -> f64 {
    let sum = signal
        .iter()
        .fold(0_f64, |acc, &sample| acc + sample.powi(2));
    let mean = sum / signal.len() as f64;

    mean.sqrt()
}

#[cfg(test)]
mod tests {
    use super::compute;
    use std::f64;
    use utils::test;

    const FLOAT_PRECISION: f64 = 0.000_000_010;

    fn test_against(dataset: &test::data::TestDataSet) -> () {
        let rms = compute(&dataset.signal);
        assert_relative_eq!(rms,
                            dataset.features.rms,
                            epsilon = f64::EPSILON,
                            max_relative = FLOAT_PRECISION);
    }

    #[test]
    fn test_rms() {
        let datasets = test::data::get_all();

        for dataset in datasets.iter() {
            test_against(dataset);
        }
    }
}
