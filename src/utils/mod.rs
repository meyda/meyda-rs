pub fn mu(exp: i32, vec: &Vec<f64>) -> f64 {
  let fraction = vec.iter()
    .enumerate()
    .fold((0_f64, 0_f64),
      |acc, x| (acc.0 + (x.0 as f64).powi(exp), acc.1 + x.1)
    );

  fraction.0 / fraction.1
}

fn bark_map(bin: f64) -> f64 {
  13.0 * (bin / 1315.8).atan() + 3.5 * (bin / 7518.0).powi(2).atan()
}

pub fn bark_scale(length: usize, sample_rate: f64, buffer_size: f64) -> Vec<f64> {
  let scale = vec![0; length];

  scale.iter()
    .enumerate()
    .map(|s| bark_map(s.0 as f64 * sample_rate / buffer_size))
    .collect()
}
