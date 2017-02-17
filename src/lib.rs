mod extractors;

pub fn get_rms(signal : &Vec<f64>) -> f64 {
  extractors::rms::compute(signal)
}

pub fn get_zcr(signal : &Vec<f64>) -> f64 {
  extractors::zcr::compute(signal)
}

pub fn get_power_spectrum(signal : &Vec<f64>) -> Vec<f64> {
  extractors::power_spectrum::compute(signal)
}

pub fn get_amp_spectrum(signal : &Vec<f64>) -> Vec<f64> {
  extractors::amp_spectrum::compute(signal)
}