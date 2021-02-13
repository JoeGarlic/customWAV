use std::default::Default;
use std::ops::{Add, AddAssign};
use std::path::Path;

use dasp::interpolate::linear::Linear;
use dasp::signal::{self, Signal};

use crate::{Error, SBResult};

pub fn decode_samples<'a, P: AsRef<Path>>(
    path: P,
    target_sample_rate: u32,
) -> SBResult<impl Iterator<Item = f32> + 'a> {
    let decoder = creak::Decoder::open(path)?;

    let info = decoder.info();

    let mut samples = Vec::new();
    for s in decoder.into_samples()? {
        samples.push(s?);
    }
    let samples = convert_to_mono_channel(samples.into_iter(), info.channels() as u32)?;

    let mut signal = signal::from_iter(samples);
    let interp = Linear::new(signal.next(), signal.next());
    let signal = signal.from_hz_to_hz(interp, info.sample_rate() as f64, target_sample_rate as f64);

    Ok(signal.until_exhausted())
}

pub fn convert_to_mono_channel<T>(
    signal: impl Iterator<Item = T>,
    input_channels: u32,
) -> SBResult<impl Iterator<Item = T>>
where
    T: Add + AddAssign + Default,
{
    let mut new_signal = Vec::new();
    let mut v = T::default();
    let mut counter = None;
    for sample in signal {
        if let Some(counter) = counter {
            if counter == 0 {
                new_signal.push(v);
                v = T::default();
            }
            v += sample;
        } else {
            v += sample;
            counter = Some(0);
        }
        counter = counter.map(|c| (c + 1) % input_channels)
    }

    if let Some(counter) = counter {
        if counter != 0 {
            return Err(Error::ChannelCoversionError(input_channels, 1));
        }
        new_signal.push(v);
    }

    Ok(new_signal.into_iter())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_not_convert_single_channel() {
        let samples = 1..=10; //1 2 3 4 5 6 7 8 9 10

        let converted = convert_to_mono_channel(samples.clone(), 1).unwrap();

        let samples: Vec<i32> = samples.collect();
        let converted: Vec<i32> = converted.collect();
        assert_eq!(converted, samples);
    }

    #[test]
    fn it_should_still_be_empty() {
        let samples = std::iter::empty(); //empty iter

        let converted = convert_to_mono_channel(samples.clone(), 1).unwrap();

        let samples: Vec<i32> = samples.collect();
        let converted: Vec<i32> = converted.collect();
        assert_eq!(converted, samples);
    }

    #[test]
    fn it_converts_dual_channel_to_mono() {
        let samples = 1..=10; //1 2 3 4 5 6 7 8 9 10

        let converted = convert_to_mono_channel(samples, 2).unwrap();

        let samples: Vec<i32> = vec![1 + 2, 3 + 4, 5 + 6, 7 + 8, 9 + 10];
        let converted: Vec<i32> = converted.collect();
        assert_eq!(converted, samples);
    }

    #[test]
    fn it_errors_with_channel_mismatch_error() {
        let samples = 1..10; //1 2 3 4 5 6 7 8 9

        let res = convert_to_mono_channel(samples, 2);

        match res {
            Err(e) => match e {
                Error::ChannelCoversionError(source, target) => {
                    assert_eq!((source, target), (2, 1))
                }
                _ => panic!("Expected ChannelConversionError"),
            },
            Ok(_) => panic!("Expected a ChannelConversionError got Ok(iter)"),
        }
    }
}
