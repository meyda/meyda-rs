/**
 * @brief      ENERGY
 *
 * @param      signal  The signal vector (Vec::<f64>)
 *
 * @return     the energy value (f64)
 */
pub fn compute(signal : &Vec<f64>) -> f64 {
  let energy = signal.iter().fold(0_f64, |acc, &sample| acc + sample.abs().powi(2));

  return energy;
}