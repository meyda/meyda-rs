use extractors::amp_spectrum;

pub fn compute(signal: &Vec<f64>) -> f64 {
    let amp_spec: Vec<f64> = amp_spectrum::compute(signal);

    let fraction = amp_spec
        .iter()
        .fold((0.0, 0.0), |acc, &x| (acc.0 + x.ln(), acc.1 + x));

    (fraction.0 / amp_spec.len() as f64).exp() * amp_spec.len() as f64 / fraction.1
}

#[cfg(test)]
mod tests {
    use super::compute;
    use std::f64;
    use utils::test;

    const FLOAT_PRECISION: f64 = 0.001_000_000;

    fn test_against(dataset: &test::data::TestDataSet) -> () {
        let sf = compute(&dataset.signal);

        assert_relative_eq!(
            sf,
            dataset.features.spectralFlatness,
            epsilon = f64::EPSILON,
            max_relative = FLOAT_PRECISION
        );
    }

    #[test]
    fn test_spectral_flatness() {
        let datasets = test::data::get_all();

        for dataset in datasets.iter() {
            test_against(dataset);
        }
    }
}
