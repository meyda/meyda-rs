use crate::extractors::amp_spectrum;
use crate::utils;

pub fn compute(signal: &Vec<f64>, sample_rate: f64) -> Vec<f64> {
    let mut amp_spec: Vec<f64> = amp_spectrum::compute(signal);

    let original_len = amp_spec.len();

    let bark_limits: Vec<usize> =
        utils::bark_limits(amp_spec.len(), sample_rate, amp_spec.len() as f64 / 2.0);

    let loudnesses = bark_limits[1..]
        .iter()
        .map(|&lim| {
            let current_limit = lim - (original_len - amp_spec.len());

            let end = match current_limit {
                e if amp_spec.len() >= e => e,
                _ => amp_spec.len(),
            };

            amp_spec.drain(0..end).sum::<f64>().powf(0.23)
        })
        .collect();

    loudnesses
}

#[cfg(test)]
mod tests {
    use super::compute;
    use std::f64;
    use utils::test;

    const FLOAT_PRECISION: f64 = 0.333_333;
    const SAMPLE_RATE: f64 = 44100f64;

    fn test_against(dataset: &test::data::TestDataSet) -> () {
        let bark_loudness = compute(&dataset.signal, SAMPLE_RATE);

        println!("{:?}", &bark_loudness);

        test::data::approx_compare_vec(&bark_loudness, &dataset.features.loudness, FLOAT_PRECISION);
    }

    #[test]
    fn test_bark_loudness() {
        let datasets = test::data::get_all();

        for dataset in datasets.iter() {
            test_against(dataset);
        }
    }
}
