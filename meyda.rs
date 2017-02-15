mod extractors;

pub fn get_rms(signal : &Vec<f64>) -> f64 {
  extractors::rms::compute(signal)
}