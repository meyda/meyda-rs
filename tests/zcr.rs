extern crate meyda;
#[macro_use]
extern crate approx;

use std::f64;

mod test_data;

const FLOAT_PRECISION: f64 = 0.000_000_010;

#[test]
fn test_zcr() {
  let valid_signal = test_data::valid_signal();
  let zcr = meyda::get_zcr(&valid_signal.signal);

  assert_relative_eq!(zcr, valid_signal.zcr, epsilon = f64::EPSILON, max_relative = FLOAT_PRECISION);
}
