use hound::{SampleFormat, WavReader, WavSpec, WavWriter};

use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

mod audio_conversion;
mod memory_buffer;

use audio_conversion::{self as ac, ReaderVariant};

#[derive(Debug)]
pub enum Error {
    NoSamples,
    DirNotFound(PathBuf),
    HoundError(hound::Error),
    FileNotFound(PathBuf),
}

pub type SBResult<T = ()> = Result<T, Error>;

impl std::convert::From<hound::Error> for Error {
    fn from(error: hound::Error) -> Self {
        Error::HoundError(error)
    }
}

// Later we can determine this on the fly
const SPEC: WavSpec = WavSpec {
    channels: 2,
    sample_rate: 44100,
    bits_per_sample: 16,
    sample_format: SampleFormat::Int,
};

pub fn join_samples_to_new_wav<P: AsRef<Path>, N: AsRef<str>>(
    name: &str,
    directory: P,
    sample_names: &[N],
) -> SBResult {
    let directory = directory.as_ref();

    if sample_names.is_empty() {
        return Err(Error::NoSamples);
    }
    if !directory.exists() || !directory.is_dir() {
        return Err(Error::DirNotFound(directory.to_path_buf()));
    }

    let out_filename = directory.join(name).with_extension("wav");
    let mut writer = WavWriter::create(out_filename, SPEC)?;

    if let Ok(readers) = get_wav_readers(directory, sample_names) {
        for reader in readers {
            append_to_writer::<i16>(&mut writer, reader)?;
        }
    }

    writer.finalize()?;

    Ok(())
}

fn append_to_writer<S: hound::Sample>(writer: &mut WavWriter<BufWriter<File>>, reader: ReaderVariant) -> SBResult {
    //append reader to writer
    for sample in reader.into_samples::<S>()? {
        writer.write_sample(sample)?;
    }
    Ok(())
}

fn get_wav_readers<P: AsRef<Path>, N: AsRef<str>>(
    directory: P,
    names: &[N],
) -> SBResult<Vec<ReaderVariant>> {
    names
        .iter()
        .map(|name| {
            let directory = directory.as_ref();
            let name = name.as_ref();
            let wav_file = directory.join(name).with_extension("wav");
            let mp3_file = directory.join(name).with_extension("mp3");
            if wav_file.exists() && wav_file.is_file() {
                return WavReader::open(wav_file)
                    .map(|r| ReaderVariant::FileReader(r))
                    .map_err(|hound_err| Error::HoundError(hound_err));
            } else if mp3_file.exists() && mp3_file.is_file() {
                return ac::file_to_wav_reader(mp3_file);
            }
            Err(Error::FileNotFound(directory.join(name)))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    //fn get_sample_paths_finds_files() {
    //let paths = get_readers("tests/samples", &["and", "fuck"]);
    //assert_eq!(
    //paths,
    //Some(vec![
    //PathBuf::from("tests/samples/and.wav"),
    //PathBuf::from("tests/samples/fuck.wav"),
    //])
    //)
    //}

    #[test]
    fn get_sample_paths_finds_none() {
        let res = get_wav_readers("dir/does/not/exist", &["and", "fuck"]);
        assert!(res.is_err());
    }

    #[test]
    fn mp3_to_wav_works() {
        assert!(ac::file_to_wav_reader("tests/samples/mp3/test.mp3").is_ok());
    }
}
