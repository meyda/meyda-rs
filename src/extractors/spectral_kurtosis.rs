use extractors::amp_spectrum;
use utils;

pub fn compute(signal: &Vec<f64>) -> f64 {
    let amp_spec: Vec<f64> = amp_spectrum::compute(signal);

    let mus: Vec<f64> = (1..5)
        .map(|x| utils::mu(x as i32, &amp_spec))
        .into_iter()
        .collect();

    let numerator =
        -3.0 * mus[0].powf(4.0) + 6.0 * mus[0] * mus[1] - 4.0 * mus[0] * mus[2] + mus[3];

    let denominator = (mus[1] - mus[0].powf(2.0)).sqrt().powf(4.0);

    numerator / denominator
}

#[cfg(test)]
mod tests {
    use super::compute;
    use std::f64;
    use utils::test;

    const FLOAT_PRECISION: f64 = 0.000_010_000;

    fn test_against(dataset: &test::data::TestDataSet) -> () {
        let sk = compute(&dataset.signal);

        assert_relative_eq!(
            sk,
            dataset.features.spectralKurtosis,
            epsilon = f64::EPSILON,
            max_relative = FLOAT_PRECISION
        );
    }

    #[test]
    fn test_spectral_kurtosis() {
        let datasets = test::data::get_all();

        for dataset in datasets.iter() {
            test_against(dataset);
        }
    }
}
