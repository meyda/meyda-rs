pub fn compute(signal : &Vec<f64>) -> f64 {
  let mut sum = 0_f64;

  for sample in signal {
    sum += sample.powf(2_f64);
  }

  sum /= signal.len() as f64;

  sum.sqrt()
}