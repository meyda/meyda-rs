pub mod test;
pub type Hz = f64;

pub fn mu(exp: i32, vec: &Vec<f64>) -> f64 {
    let fraction = vec.iter()
        .enumerate()
        .fold((0_f64, 0_f64),
              |acc, x| (acc.0 + (x.0 as f64).powi(exp), acc.1 + x.1));

    fraction.0 / fraction.1
}

fn bark_map(freq: Hz) -> Hz {
    13.0 * (freq / 1315.8).atan() + 3.5 * (freq / 7518.0).powi(2).atan()
}

fn bark_scale(length: usize, sample_rate: Hz, buffer_size: f64) -> Vec<Hz> {
    vec![0.0 as Hz; length]
        .iter()
        .enumerate()
        .map(|s| bark_map(s.0 as Hz * sample_rate / buffer_size))
        .collect()
}

fn find_limit(ref scale: &Vec<Hz>, bark_freq: Hz) -> usize {
    let found = scale.iter().position(|&freq| freq >= bark_freq);

    match found {
        Some(lim) => lim,
        None => scale.len() - 1,
    }
}

pub fn bark_limits(spec_size: usize, sample_rate: f64, buffer_size: f64) -> Vec<usize> {
    let num_bands = 24;

    let scale = bark_scale(spec_size, sample_rate as Hz, buffer_size);

    let band_width: Hz = match scale.last() {
        Some(&freq) => freq / num_bands as f64,
        None => 0.0,
    };

    let limits = 0..num_bands;

    limits
        .map(|band| {
                 match band {
                     0 => band, // first limit is 0
                     _ => find_limit(&scale, band as Hz * band_width),
                 }
             })
        .collect()
}
