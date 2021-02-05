use hound::{SampleFormat, WavReader, WavSpec, WavWriter};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

type Reader = WavReader<BufReader<File>>;
type Writer = WavWriter<BufWriter<File>>;

#[derive(Debug)]
pub enum Error {
    NoSamples,
    DirNotFound(PathBuf),
    HoundError(hound::Error),
}

pub type SBResult< T = ()> = Result<T, Error>;

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

    if let Some(files) = get_sample_paths(directory, sample_names) {
        for file in files {
            let reader = WavReader::open(file)?;
            append_to_writer(&mut writer, reader)?;
        }
    }

    writer.finalize()?;

    Ok(())
}

fn append_to_writer(writer: &mut Writer, reader: Reader) -> SBResult {
    //append reader to writer
    for sample in reader.into_samples::<i16>() {
        writer.write_sample(sample?)?;
    }

    Ok(())
}

fn get_sample_paths<P: AsRef<Path>, N: AsRef<str>>(
    directory: P,
    names: &[N],
) -> Option<Vec<PathBuf>> {
    names
        .iter()
        .map(|name| {
            let file = directory.as_ref().join(name.as_ref()).with_extension("wav");
            if file.exists() && file.is_file() {
                return Some(file);
            }
            None
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn get_sample_paths_finds_files() {
        let paths = get_sample_paths("tests/samples", &["and", "fuck"]);
        assert_eq!(
            paths,
            Some(vec![
                PathBuf::from("tests/samples/and.wav"),
                PathBuf::from("tests/samples/fuck.wav"),
            ])
        )
    }

    #[test]
    fn get_sample_paths_finds_none() {
        let paths = get_sample_paths("tests", &["and", "fuck"]);
        assert_eq!(paths, None,)
    }

}
