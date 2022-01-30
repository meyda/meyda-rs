#[cfg(test)]
#[macro_use]
extern crate serde_derive;
#[cfg(test)]
#[macro_use]
extern crate approx;

mod extractors;
mod utils;

pub type Hz = utils::Hz;

pub fn get_rms(signal: &Vec<f64>) -> f64 {
    extractors::rms::compute(signal)
}

pub fn get_energy(signal: &Vec<f64>) -> f64 {
    extractors::energy::compute(signal)
}

pub fn get_zcr(signal: &Vec<f64>) -> f64 {
    extractors::zcr::compute(signal)
}

pub fn get_amp_spectrum(signal: &Vec<f64>) -> Vec<f64> {
    extractors::amp_spectrum::compute(signal)
}

pub fn get_power_spectrum(signal: &Vec<f64>) -> Vec<f64> {
    extractors::power_spectrum::compute(signal)
}

pub fn get_spectral_centroid(signal: &Vec<f64>) -> f64 {
    extractors::spectral_centroid::compute(signal)
}

pub fn get_spectral_flatness(signal: &Vec<f64>) -> f64 {
    extractors::spectral_flatness::compute(signal)
}

pub fn get_spectral_kurtosis(signal: &Vec<f64>) -> f64 {
    extractors::spectral_kurtosis::compute(signal)
}

pub fn get_spectral_rolloff(signal: &Vec<f64>,
                            sample_rate: f64,
                            rolloff_point: Option<f64>)
                            -> f64 {
    extractors::spectral_rolloff::compute(signal, sample_rate, rolloff_point)
}

pub fn get_bark_loudness(signal: &Vec<f64>, sample_rate: f64) -> Vec<f64> {
    extractors::bark_loudness::compute(signal, sample_rate)
}

/// Spectral Slope
///
/// A measure of how ‘inclined’ the shape of the spectrum is. Calculated by
/// performing linear regression on the amplitude spectrum.
///
/// Can be used to differentiate between different voice qualities, such as
/// hissing, breathing and regular speech. Closely relates to spectral centroid
/// and spectral rolloff.
///
/// ### Params
///
/// - `signal` The signal vector (`Vec<f64>`)
/// - `sample_rate` The sample rate of the signal (Hz)
pub fn get_spectral_slope(signal: &Vec<f64>, sample_rate: f64) -> f64 {
    extractors::spectral_slope::compute(signal, sample_rate)
}
