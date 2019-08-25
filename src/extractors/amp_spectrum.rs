extern crate num_complex;
extern crate rustfft;
/**
 * @brief      AMPLITUDE SPECTRUM
 *
 * @param      signal  The signal vector (Vec::<f64>)
 *
 * @return     The amplitude spectrum vector (Vec::<f64>)
 */
pub fn compute(signal: &Vec<f64>) -> Vec<f64> {
    let fft_len = signal.len();
    let fft = rustfft::FFTplanner::new(false).plan_fft(fft_len);

    let mut complex_signal: Vec<_> = signal
        .iter()
        .map(|&sample| num_complex::Complex::new(sample, 0_f64))
        .collect();

    let mut spectrum = vec![num_complex::Complex { re: 0.0, im: 0.0 }; fft_len];

    fft.process(&mut complex_signal, &mut spectrum);

    let amp_spectrum: Vec<f64> = spectrum
        .iter()
        .take(spectrum.len() / 2)
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
