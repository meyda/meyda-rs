pub fn mu(exp: i32, vec: &Vec<f64>) -> f64 {
  let fraction = vec.iter()
    .enumerate()
    .fold((0_f64, 0_f64),
      |acc, x| (acc.0 + (x.0 as f64).powi(exp), acc.1 + x.1)
    );

  fraction.0 / fraction.1
}