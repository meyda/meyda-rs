/**
 * @brief      ZERO CROSSING RATE
 *
 * @param      signal  The signal vector (Vec::<f64>)
 *
 * @return     the zero crossing rate (f64)
 */
pub fn compute(signal : &Vec<f64>) -> f64 {
  // here stats is a tuple, first element being the previous sample and next element being the ZCR accumulator
  let zcr_tuple = signal.iter()
  .fold((0_f64, 0_f64),
    |stats, &sample| (sample, stats.1 + if stats.0.signum() != sample.signum() { 1_f64 } else { 0_f64 })
  );

  return zcr_tuple.1;
}

#[cfg(test)]
mod tests {
  use super::compute;
  use std::f64;
  use utils::test;

  const FLOAT_PRECISION: f64 = 0.000_000_010;

  fn test_against(dataset: &test::data::TestDataSet) -> () {
    let zcr = compute(&dataset.signal);
    assert_relative_eq!(zcr, dataset.features.zcr, epsilon = f64::EPSILON, max_relative = FLOAT_PRECISION);
  }

  #[test]
  fn test_zcr() {
    let datasets = test::data::get_all();

    for dataset in datasets.iter() {
      test_against(dataset);
    }
  }
}
