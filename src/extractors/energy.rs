pub fn compute(signal: &Vec<f64>) -> f64 {
    let energy = signal
        .iter()
        .fold(0_f64, |acc, &sample| acc + sample.abs().powi(2));

    return energy;
}

#[cfg(test)]
mod tests {
    use super::compute;
    use crate::utils::test;
    use std::f64;

    const FLOAT_PRECISION: f64 = 0.000_000_010;

    fn test_against(dataset: &test::data::TestDataSet) -> () {
        let energy = compute(&dataset.signal);
        assert_relative_eq!(
            energy,
            dataset.features.energy,
            epsilon = f64::EPSILON,
            max_relative = FLOAT_PRECISION
        );
    }

    #[test]
    fn test_energy() {
        let datasets = test::data::get_all();

        for dataset in datasets.iter() {
            test_against(dataset);
        }
    }
}
