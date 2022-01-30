use extractors::amp_spectrum;
use utils;

pub fn compute(signal: &Vec<f64>) -> f64 {
    let amp_spec: Vec<f64> = amp_spectrum::compute(signal);

    utils::mu(1, &amp_spec)
}

#[cfg(test)]
mod tests {
    use super::compute;
    use std::f64;
    use utils::test;

    const FLOAT_PRECISION: f64 = 0.000_001_000;

    fn test_against(dataset: &test::data::TestDataSet) -> () {
        let sc = compute(&dataset.signal);

        assert_relative_eq!(
            sc,
            dataset.features.spectralCentroid,
            epsilon = f64::EPSILON,
            max_relative = FLOAT_PRECISION
        );
    }

    #[test]
    fn test_spectral_centroid() {
        let datasets = test::data::get_all();

        for dataset in datasets.iter() {
            test_against(dataset);
        }
    }
}
