/**
 * @brief      ROOT MEAN SQUARE
 *
 * @param      signal  The signal vector (Vec::<f64>)
 *
 * @return     the RMS value (f64)
 */
pub fn compute(signal : &Vec<f64>) -> f64 {
  let sum = signal.iter().fold(0_f64, |acc, &sample| acc + sample.powi(2));
  let mean = sum / signal.len() as f64;

  mean.sqrt()
}