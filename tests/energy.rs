extern crate meyda;
#[macro_use]
extern crate approx;

use std::f64;

mod test_data;

const FLOAT_PRECISION: f64 = 0.000_000_010;

#[test]
fn test_energy() {
  let valid_signal = test_data::valid_signal();
  let energy = meyda::get_energy(&valid_signal.signal);

  assert_relative_eq!(energy, valid_signal.energy, epsilon = f64::EPSILON, max_relative = FLOAT_PRECISION);
}
