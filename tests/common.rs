#[allow(dead_code)]
pub const SPEC_8K_RATE: hound::WavSpec = hound::WavSpec {
    bits_per_sample: 32,
    channels: 1,
    sample_format: hound::SampleFormat::Float,
    sample_rate: 8000,
};

#[allow(dead_code)]
pub const SPEC_441K_RATE: hound::WavSpec = hound::WavSpec {
    bits_per_sample: 32,
    channels: 1,
    sample_format: hound::SampleFormat::Float,
    sample_rate: 44100,
};

#[allow(dead_code)]
pub const TEST_WAV_SPEC_2: hound::WavSpec = hound::WavSpec {
    bits_per_sample: 32,
    channels: 1,
    sample_format: hound::SampleFormat::Float,
    sample_rate: 16000,
};

pub fn expected_wav_samples() -> impl Iterator<Item = f32> {
    (1..=16000).map(|i| f32::sin(i as f32))
}

pub fn new_wav_file() -> tempfile::NamedTempFile {
    let mut file = tempfile::Builder::new().suffix(".wav").tempfile().unwrap();
    let mut wav_writer = hound::WavWriter::new(&mut file, SPEC_8K_RATE).unwrap();

    for i in expected_wav_samples() {
        wav_writer.write_sample(i).unwrap();
    }

    drop(wav_writer);
    file
}

pub fn samples_per_expected_wav_file() -> usize {
    expected_wav_samples().count()
}

pub fn new_mp3_file() -> tempfile::NamedTempFile {
    let temp_file = tempfile::Builder::new().suffix(".mp3").tempfile().unwrap();

    // copy contents of our Pre-recorded sin-wav mp3 file to a new temp file for testing
    // In the future I want to be able to encode mp3, but no pure crates exist for that yet :(
    std::fs::copy("tests/res/test.mp3", temp_file.path()).expect("I can copy the file");
    temp_file
}

pub fn expected_mp3_samples() -> impl Iterator<Item = f32> {
    use std::fs::File;
    use std::io::Read;
    let mut f = File::open("tests/res/test_mp3_samples.serde").unwrap();
    let mut sample_string = String::new();
    f.read_to_string(&mut sample_string).unwrap();

    let samples: Vec<f32> = serde_json::from_str(&sample_string).unwrap();

    samples.into_iter()
}

pub fn samples_per_expected_mp3_file() -> usize {
    expected_mp3_samples().count()
}
