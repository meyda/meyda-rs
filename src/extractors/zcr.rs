pub fn compute(signal : &Vec<f64>) -> f64 {
  // here stats is a tuple, first element being the previous sample and next element being the ZCR accumulator
  let zcr_tuple = signal.iter().fold((0_f64, 0_f64), |stats, &sample| {
    (sample, stats.1 + if (stats.0 >= 0_f64 && sample < 0_f64) || (stats.0 < 0_f64 && sample >= 0_f64) { 1_f64 } else { 0_f64 })
  });

  return zcr_tuple.1;
}