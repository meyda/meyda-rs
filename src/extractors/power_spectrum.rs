use extractors::amp_spectrum;

pub fn compute(signal: &Vec<f64>) -> Vec<f64> {
    let amp_spec: Vec<f64> = amp_spectrum::compute(signal);

    let pow_spec: Vec<f64> = amp_spec.iter().map(|bin| bin.powi(2)).into_iter().collect();

    return pow_spec;
}
