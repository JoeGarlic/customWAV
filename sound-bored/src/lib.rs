use hound::{SampleFormat, WavReader, WavSpec, WavWriter};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

type Reader = WavReader<BufReader<File>>;
type Writer = WavWriter<BufWriter<File>>;

//type SoundBoredResult<T = ()> = Result<T, Error>;

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
) -> hound::Result<()> {
    let directory = directory.as_ref();

    if !directory.exists() || !directory.is_dir() {
        todo!("Perform error handling.")
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

fn append_to_writer(writer: &mut Writer, reader: Reader) -> Result<(), hound::Error> {
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

//// read and append
//let reader = WavReader::open("audio-samples/the.wav").unwrap();
//append_to_writer(&mut writer, reader).expect("appended");

//Ok(())
//
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
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

    #[test]
    fn generated_file_is_always_the_same() {
        let custom_audio_filename = "tests/samples/cross_check_hasher.wav";
        let known_good_filename = "tests/samples/hasher_base.wav";
        let fuck_audio_filename = "audio-samples/fuck.wav";

        let mut writer = WavWriter::create(custom_audio_filename, SPEC).unwrap();

        // read and append
        let reader = WavReader::open(fuck_audio_filename).unwrap();
        append_to_writer(&mut writer, reader).expect("appended");

        writer.finalize().unwrap();

        let new = WavReader::open(custom_audio_filename).unwrap();
        let good = WavReader::open(known_good_filename).unwrap();

        let new_hash = md5::compute(
            new.into_inner()
                .bytes()
                .map(|res| res.unwrap())
                .collect::<Vec<u8>>(),
        );
        let good_hash = md5::compute(
            good.into_inner()
                .bytes()
                .map(|res| res.unwrap())
                .collect::<Vec<u8>>(),
        );

        assert_eq!(new_hash, good_hash);
    }
}
