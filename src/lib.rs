use std::path::{Path, PathBuf};

use creak;
use dasp::interpolate::linear::Linear;
use dasp::signal::{self, Signal};
use hound;

#[derive(Debug)]
pub enum Error {
    FileNotFound(PathBuf, String),
    DirectoryNotFound(PathBuf),
    CreakErr(creak::DecoderError),
    HoundErr(hound::Error),
    NoSamples,
}

impl std::convert::From<hound::Error> for Error {
    fn from(error: hound::Error) -> Self {
        Error::HoundErr(error)
    }
}

impl std::convert::From<creak::DecoderError> for Error {
    fn from(error: creak::DecoderError) -> Self {
        Error::CreakErr(error)
    }
}

pub type SBResult<T = ()> = Result<T, Error>;

fn decode_samples<'a, P: AsRef<Path>>(
    path: P,
    target_sample_rate: u32,
) -> SBResult<impl Iterator<Item = f32> + 'a> {
    let decoder = creak::Decoder::open(path)?;

    let info = decoder.info();
    let mut samples = Vec::new();

    let channels = info.channels();

    let mut v = 0.0;
    let mut counter = None;
    for sample in decoder.into_samples()? {
        let sample = sample?;

        if let Some(counter) = counter {
            if counter == 0 {
                samples.push(v);
                v = 0.0;
            }
            v += sample;
        } else {
            v += sample;
            counter = Some(0);
        }
        counter = counter.map(|c| (c + 1) % channels)
    }

    let mut signal = signal::from_iter(samples);
    let interp = Linear::new(signal.next(), signal.next());
    let signal = signal.from_hz_to_hz(interp, info.sample_rate() as f64, target_sample_rate as f64);

    Ok(signal.until_exhausted())
}

pub fn join_files<P: AsRef<Path>>(
    paths: &[P],
    target_sample_rate: u32,
) -> SBResult<impl Iterator<Item = f32> + '_> {
    let mut samples = Vec::new();
    for path in paths {
        samples.push(decode_samples(path, target_sample_rate)?)
    }

    Ok(samples.into_iter().flatten())
}

pub fn join_samples_to_new_wav<P: AsRef<Path>, N: AsRef<str>>(
    name: &str,
    directory: P,
    sample_names: &[N],
    sample_rate: u32,
) -> SBResult {
    let directory = directory.as_ref();

    if sample_names.is_empty() {
        return Err(Error::NoSamples);
    }

    let out_filename = directory.join(name).with_extension("wav");

    let paths = get_audio_file_paths(directory, sample_names)?;

    let audio = join_files(&paths, sample_rate)?;
    let mut writer = hound::WavWriter::create(
        out_filename,
        hound::WavSpec {
            channels: 1,
            sample_rate,
            sample_format: hound::SampleFormat::Float,
            bits_per_sample: 32,
        },
    )?;

    for sample in audio {
        writer.write_sample(sample)?;
    }
    writer.finalize()?;
    Ok(())
}

pub fn get_audio_file_paths<P: AsRef<Path>, S: AsRef<str>>(
    directory: P,
    names: &[S],
) -> SBResult<Vec<PathBuf>> {
    let directory = directory.as_ref();

    if !directory.exists() || !directory.is_dir() {
        return Err(Error::DirectoryNotFound(directory.to_path_buf()));
    }

    let mut paths = Vec::new();
    for name in names {
        let name = name.as_ref();
        let p = directory.join(name);
        if let Ok(meta) = p.with_extension("wav").metadata() {
            if meta.is_file() {
                paths.push(p.with_extension("wav"));
            }
        } else if let Ok(meta) = p.with_extension("mp3").metadata() {
            if meta.is_file() {
                paths.push(p.with_extension("mp3"));
            }
        } else {
            return Err(Error::FileNotFound(
                directory.to_path_buf(),
                name.to_string(),
            ));
        }
    }

    Ok(paths)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn it_should_get_a_wav_file() {
        let dir = tempfile::tempdir().unwrap();
        File::create(dir.path().join("test.wav")).unwrap();

        let paths = get_audio_file_paths(dir.path(), &vec!["test"]).unwrap();

        assert_eq!(paths, &[dir.path().join("test.wav")]);
    }

    #[test]
    fn it_should_get_an_mp3_file() {
        let dir = tempfile::tempdir().unwrap();
        File::create(dir.path().join("test.mp3")).unwrap();

        let paths = get_audio_file_paths(dir.path(), &vec!["test"]).unwrap();

        assert_eq!(paths, &[dir.path().join("test.mp3")]);
    }

    #[test]
    fn it_should_get_an_mp3_and_a_wav() {
        let dir = tempfile::tempdir().unwrap();
        File::create(dir.path().join("mp3.mp3")).unwrap();
        File::create(dir.path().join("wav.wav")).unwrap();

        let paths = get_audio_file_paths(dir.path(), &vec!["wav", "mp3"]).unwrap();

        assert_eq!(
            paths,
            &[dir.path().join("wav.wav"), dir.path().join("mp3.mp3")]
        );
    }

    #[test]
    fn it_should_get_wav_over_mp3() {
        let dir = tempfile::tempdir().unwrap();
        File::create(dir.path().join("test.mp3")).unwrap();
        File::create(dir.path().join("test.wav")).unwrap();

        let paths = get_audio_file_paths(dir.path(), &vec!["test"]).unwrap();

        assert_eq!(paths, &[dir.path().join("test.wav")]);
    }

    #[test]
    fn it_should_error_with_file_not_found() {
        let dir = tempfile::tempdir().unwrap();

        let res = get_audio_file_paths(dir.path(), &vec!["test"]);

        match res.expect_err("Expected an error") {
            Error::FileNotFound(directory, name) => assert_eq!(
                (directory, name),
                (dir.path().to_path_buf(), String::from("test"))
            ),
            _ => panic!("Expected FileNotFound Error"),
        }
    }

    #[test]
    fn it_should_error_with_diretory_not_found() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().to_path_buf();
        drop(dir);

        let res = get_audio_file_paths(&path, &vec!["test"]);

        match res.expect_err("Expected an error") {
            Error::DirectoryNotFound(p) => assert_eq!(p, path),
            _ => panic!("Expected DirectoryNotFound Error"),
        }
    }

    //#[test]
    //fn it_should_not_convert_single_channel() {
    //    let samples = 1..=10; //1 2 3 4 5 6 7 8 9 10

    //    let converted = convert_to_mono_channel_signal(samples.clone(), 1);

    //    let samples: Vec<i32> = samples.collect();
    //    let converted: Vec<i32> = converted.collect();
    //    assert_eq!(converted, samples);
    //}

}
