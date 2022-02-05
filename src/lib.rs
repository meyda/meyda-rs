//! Meyda is a Rust library for audio feature extraction. It's based on [the
//! original library in Javascript](https://meyda.js.org). It implements a
//! variety of well known audio feature extraction algorithms.

#[cfg(test)]
#[macro_use]
extern crate serde_derive;
#[cfg(test)]
#[macro_use]
extern crate approx;

mod extractors;
mod utils;

/// A type alias representing hertz
pub type Hz = utils::Hz;

///
/// Root Mean Square
///
/// The root mean square of the waveform, that corresponds to its loudness.
/// Corresponds to the `Energy` feature in YAAFE, adapted from Loy’s
/// Musimathics.
///
/// RMS is used for information about the loudness of the sound.
///
/// G. Loy, Musimathics: The Mathematical Foundations of Music, Volume 1. The
/// MIT Press, 2006.
///
/// ### Params:
///
/// - `signal`  The signal vector (`Vec<f64>`)
///
/// Returns the root mean square of the signal.
pub fn get_rms(signal: &Vec<f64>) -> f64 {
    extractors::rms::compute(signal)
}

/// Energy
///
/// The infinite integral of the squared signal. According to Lathi.
///
/// B. P. Lathi, Modern Digital and Analog Communication Systems 3e Osece.
/// Oxford University Press, 3rd ed., 1998.
///
/// ### Params:
///
/// - `signal`  The signal vector (`Vec<f64>`)
pub fn get_energy(signal: &Vec<f64>) -> f64 {
    extractors::energy::compute(signal)
}

/// Zero Crossing Rate
///
/// The number of times that the signal crosses the zero value in the buffer.
///
/// Used for differentiating between percussive and pitched sounds. Percussive
/// sounds will have a random ZCR across buffers, whereas pitch sounds will
/// return a more constant value.
///
/// ### Params:
///
/// - `signal`  The signal vector (`Vec<f64>`)
pub fn get_zcr(signal: &Vec<f64>) -> f64 {
    extractors::zcr::compute(signal)
}

/// Amplitude Spectrum
///
/// This is also known as the magnitude spectrum. By calculating the Fast
/// Fourier Transform (FFT), we get the signal represented in the frequency
/// domain. The output is an array, where each index is a frequency bin (i.e.
/// containing information about a range of frequencies) containing a complex
/// value (real and imaginary). The amplitude spectrum takes this complex array
/// and computes the amplitude of each index. The result is the distribution of
/// frequencies in the signal along with their corresponding strength. If you
/// want to learn more about Fourier Transform, and the differences between
/// time-domain to frequency-domain analysis, [this
/// article](https://www.mathworks.com/help/signal/examples/practical-introduction-to-frequency-domain-analysis.html)
/// is a good place to start.
///
/// ### Params:
///
/// - `signal`  The signal vector (`Vec<f64>`)
pub fn get_amp_spectrum(signal: &Vec<f64>) -> Vec<f64> {
    extractors::amp_spectrum::compute(signal)
}

/// Power Spectrum
///
/// This is the `amplitudeSpectrum` squared.
///
/// ### Params:
///
/// - `signal`  The signal vector (`Vec<f64>`)
pub fn get_power_spectrum(signal: &Vec<f64>) -> Vec<f64> {
    extractors::power_spectrum::compute(signal)
}

/// Spectral Centroid
///
/// An indicator of the "brightness" of a given sound, representing the spectral
/// centre of gravity. If you were to take the spectrum, make a wooden block out
/// of it and try to balance it on your finger (across the X axis), the spectral
/// centroid would be the frequency that your finger "touches" when it
/// successfully balances.
///
/// ### Params:
///
/// - `signal`  The signal vector (`Vec<f64>`)
pub fn get_spectral_centroid(signal: &Vec<f64>) -> f64 {
    extractors::spectral_centroid::compute(signal)
}

/// Spectral Flatness
///
/// The flatness of the spectrum. It is computed using the ratio between the
/// geometric and arithmetic means.
///
/// ### Params:
///
/// - `signal`  The signal vector (`Vec<f64>`)
pub fn get_spectral_flatness(signal: &Vec<f64>) -> f64 {
    extractors::spectral_flatness::compute(signal)
}

/// Spectral Kurtosis
///
/// A measure of how quickly the spectrum of a signal is changing. It is
/// calculated by computing the difference between the current spectrum and that
/// of the previous frame.
///
/// ### Params:
///
/// - `signal`  The signal vector (`Vec<f64>`)
pub fn get_spectral_kurtosis(signal: &Vec<f64>) -> f64 {
    extractors::spectral_kurtosis::compute(signal)
}

/// Spectral Rolloff
///
/// The frequency below which is contained a given % of the energy of the
/// spectrum.
///
/// ### Params:
///
/// - `signal` The signal vector (`Vec<f64>`)
/// - `sample_rate` The sample rate of the signal (Hz)
/// - `rolloff_point` the given percentage (default: 0.99)
pub fn get_spectral_rolloff(
    signal: &Vec<f64>,
    sample_rate: f64,
    rolloff_point: Option<f64>,
) -> f64 {
    extractors::spectral_rolloff::compute(signal, sample_rate, rolloff_point)
}

/// Specific Bark Loudness
///
/// Humans' perception of frequency is non-linear. The [Bark
/// Scale](https://en.wikipedia.org/wiki/Bark_scale) was developed in order to
/// have a scale on which equal distances correspond to equal distances of
/// frequency perception.
///
/// Returns the loudness of the input sound on each step (often referred to as
/// bands) of this scale (`.specific`). There are 24 bands overall.
///
/// Takes a signal vector `signal` (`Vec<f64>`) and the sample rate of the
/// signal `sample_rate` (`f64`), and returns the Loudness in each Bark band
/// `Vec<f64>`.
///
/// ### Params:
///
/// - `signal` The signal vector (`Vec<f64>`)
/// - `sample_rate` The sample rate of the signal (Hz)
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
