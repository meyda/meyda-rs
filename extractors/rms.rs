pub fn compute(signal : &Vec<f64>) -> f64 {
  let sum = signal.iter().fold(0_f64, |acc, &sample| acc + sample.powf(2_f64));
  let mean = sum / signal.len() as f64;

  mean.sqrt()
}