use hound::{SampleFormat, WavReader, WavSpec, WavWriter};
use std::fs::File;
use std::io::{BufReader, BufWriter};

type Reader = WavReader<BufReader<File>>;
type Writer = WavWriter<BufWriter<File>>;

fn main() -> Result<(), hound::Error> {
    let spec = WavSpec {
        channels: 2,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

    let custom_audio_filename = "audio-samples/custom.wav";
    let fuck_audio_filename = "audio-samples/fuck.wav";

    // overwrite custom file with empty wav.
    let mut writer = WavWriter::create(custom_audio_filename, spec).unwrap();

    // read and append
    //let reader = WavReader::open(fuck_audio_filename).unwrap();
    //append_to_writer(&mut writer, reader).expect("appended");

     //read and append
    //let reader = WavReader::open("audio-samples/the.wav").unwrap();
    //append_to_writer(&mut writer, reader).expect("appended");

    Ok(())
}

