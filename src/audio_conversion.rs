use std::fs::File;
use std::io::{self, BufReader, Seek};
use std::path::Path;

use hound::{SampleFormat, WavReader, WavSpec, WavWriter};

use crate::memory_buffer::MemoryByteBuffer;
use crate::SBResult;

pub enum ReaderVariant {
    FileReader(WavReader<BufReader<File>>),
    MemoryReader(WavReader<MemoryByteBuffer>),
}

impl ReaderVariant {
    pub fn into_samples<S: hound::Sample>(self) -> SBResult<Vec<S>> {
        match self {
            ReaderVariant::FileReader(reader) => {
                let mut v = Vec::new();
                for s in reader.into_samples::<S>() {
                    let s = s?;
                    v.push(s);
                }
                Ok(v)
            }
            ReaderVariant::MemoryReader(reader) => {
                let mut v = Vec::new();
                for s in reader.into_samples::<S>() {
                    let s = s?;
                    v.push(s);
                }
                Ok(v)
            }
        }
    }
}

pub fn file_to_wav_reader<P: AsRef<Path>>(mp3_file_path: P) -> SBResult<ReaderVariant> {
    let mp3_file_path = mp3_file_path.as_ref();

    let file = std::fs::File::open(mp3_file_path).expect("mp3_file didn't open");
    let (header, samples) = puremp3::read_mp3(file).unwrap();
    dbg!(&header);

    let spec = WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

    let mut wav_buffer = MemoryByteBuffer::new();
    let mut wav_writer = WavWriter::new(&mut wav_buffer, spec).unwrap();

    for (i, sample) in samples.enumerate() {
        dbg!(i);
        dbg!(&sample.0);
        wav_writer.write_sample(float_sample_to_int_sample(sample.0)).unwrap();
        wav_writer.write_sample(float_sample_to_int_sample(sample.1)).unwrap();
    }

    wav_writer.finalize().unwrap();
    wav_buffer.seek(io::SeekFrom::Start(0)).unwrap();

    Ok(ReaderVariant::MemoryReader(
        WavReader::new(wav_buffer).unwrap(),
    ))
}

pub fn float_sample_to_int_sample(mut f: f32) -> i16 {
    //float f = ...;
    //int16 i;
    //f = f * 32768 ;
    //if( f > 32767 ) f = 32767;
    //if( f < -32768 ) f = -32768;
    //i = (int16) f;
    f = f * 32768.0;
    if f > i16::MAX as f32 {
        f = i16::MAX as f32;
    }
    if f < i16::MIN as f32 {
        f = i16::MIN as f32;
    }
    f as i16
}

#[cfg(test)]
mod tests {}
