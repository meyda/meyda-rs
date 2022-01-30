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

#[cfg(test)]
mod tests {
    use super::compute;
    use std::f64;
    use utils::test;

    const FLOAT_PRECISION: f64 = 0.333_333;

    fn test_against(dataset: &test::data::TestDataSet) -> () {
        let power_spec = compute(&dataset.signal);

        test::data::approx_compare_vec(
            &power_spec,
            &dataset.features.powerSpectrum,
            FLOAT_PRECISION,
        );
    }

    #[test]
    fn test_power_spectrum() {
        let datasets = test::data::get_all();

        for dataset in datasets.iter() {
            test_against(dataset);
        }
    }
}
