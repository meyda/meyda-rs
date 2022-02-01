extern crate num_complex;
extern crate rustfft;

pub fn compute(signal: &Vec<f64>) -> Vec<f64> {
    let fft_len = signal.len();
    let fft = rustfft::FftPlanner::new().plan_fft_forward(fft_len);

    let mut fft_buffer: Vec<_> = signal
        .iter()
        .map(|&sample| num_complex::Complex::new(sample, 0_f64))
        .collect();

    fft.process(&mut fft_buffer);

    let amp_spectrum: Vec<f64> = fft_buffer
        .iter()
        .take(fft_buffer.len() / 2)
        .map(|bin| {
            let tmp = bin.re.powf(2_f64) + bin.im.powf(2_f64);
            tmp.sqrt()
        })
        .collect();

    return amp_spectrum;
}

#[cfg(test)]
mod tests {
    use super::compute;
    use std::f64;
    use utils::test;

    const FLOAT_PRECISION: f64 = 0.333_333;

    fn test_against(dataset: &test::data::TestDataSet) -> () {
        let amp_spec = compute(&dataset.signal);

        test::data::approx_compare_vec(
            &amp_spec,
            &dataset.features.amplitudeSpectrum,
            FLOAT_PRECISION,
        );
    }

    #[test]
    fn test_amplitude_spectrum() {
        let datasets = test::data::get_all();

        for dataset in datasets.iter() {
            test_against(dataset);
        }
    }
}
