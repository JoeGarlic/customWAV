use creak;
use sound_bored;

mod common;

fn fuzzy_assert(value: usize, target: usize, alpha: usize) {
    eprintln!(
        "Value: {}, Acceptable Range: {} +/- {}",
        value, target, alpha
    );
    assert!(
        target - alpha <= value && value <= target + alpha,
        "Value outside of acceptable range"
    );
}

#[test]
fn join_one_wav() {
    let first_wav_file = common::new_wav_file();

    let paths = vec![first_wav_file.path()];
    let joined = sound_bored::join_files(&paths, 8000).unwrap();

    fuzzy_assert(joined.count(), common::samples_per_expected_wav_file(), 2);
}

#[test]
fn join_two_wavs() {
    let first_wav_file = common::new_wav_file();
    let second_wav_file = common::new_wav_file();

    let paths = vec![first_wav_file.path(), second_wav_file.path()];
    let joined = sound_bored::join_files(&paths, 8000).unwrap();

    fuzzy_assert(
        joined.count(),
        common::samples_per_expected_wav_file() * 2,
        4,
    );
}

#[test]
fn join_three_wavs() {
    let first_wav_file = common::new_wav_file();
    let second_wav_file = common::new_wav_file();
    let third_wav_file = common::new_wav_file();

    let paths = vec![
        first_wav_file.path(),
        second_wav_file.path(),
        third_wav_file.path(),
    ];
    let joined = sound_bored::join_files(&paths, 8000).unwrap();

    fuzzy_assert(
        joined.count(),
        common::samples_per_expected_wav_file() * 3,
        6,
    );
}

#[test]
fn join_one_mp3() {
    let mp3_file = common::new_mp3_file();
    let paths = vec![mp3_file.path()];
    let joined = sound_bored::join_files(&paths, 8000).unwrap();

    fuzzy_assert(joined.count(), common::samples_per_expected_mp3_file(), 2);
}

#[test]
fn join_two_mp3() {
    let first_mp3_file = common::new_mp3_file();
    let second_mp3_file = common::new_mp3_file();
    let paths = vec![first_mp3_file.path(), second_mp3_file.path()];
    let joined = sound_bored::join_files(&paths, 8000).unwrap();

    fuzzy_assert(
        joined.count(),
        common::samples_per_expected_mp3_file() * 2,
        4,
    );
}

#[test]
fn join_mp3_and_wav() {
    let mp3 = common::new_mp3_file();
    let wav = common::new_wav_file();
    let paths = vec![mp3.path(), wav.path()];
    let joined = sound_bored::join_files(&paths, 8000).unwrap();

    fuzzy_assert(
        joined.count(),
        common::samples_per_expected_mp3_file() + common::samples_per_expected_wav_file(),
        4,
    );
}

#[test]
fn double_sample_rate() {
    let wav = common::new_wav_file();
    let paths = vec![wav.path()];
    let joined = sound_bored::join_files(&paths, 16000).unwrap();

    fuzzy_assert(
        joined.count(),
        common::samples_per_expected_wav_file() * 2,
        4,
    );
}

#[test]
#[ignore]
fn write_join_mp3_and_wav() {
    let mp3 = common::new_mp3_file();
    let wav = common::new_wav_file();
    let paths = vec![mp3.path(), wav.path()];
    let joined = sound_bored::join_files(&paths, 8000).unwrap();

    let mut writer = hound::WavWriter::create("tests/out/out.wav", common::SPEC_8K_RATE).unwrap();
    for sample in joined {
        writer.write_sample(sample).unwrap();
    }
}

#[test]
#[ignore]
fn write_test_mp3_samples() {
    use std::fs::File;
    use std::io::Write;

    let dec = creak::Decoder::open("tests/res/test.mp3").unwrap();
    let samples: Vec<f32> = dec.into_samples().unwrap().map(|s| s.unwrap()).collect();
    let samples = serde_json::to_string(&samples).unwrap();

    let mut f = File::create("tests/res/test_mp3_samples.serde").unwrap();
    f.write(samples.as_bytes()).unwrap();
}

#[test]
#[ignore]
fn write_fuck_wav() {
    let samples = sound_bored::join_files(&["tests/res/fuck.wav"], 44100).unwrap();

    let mut writer = hound::WavWriter::create("tests/out/out.wav", common::SPEC_441K_RATE).unwrap();
    for sample in samples {
        writer.write_sample(sample).unwrap();
    }

    assert!(false);
}
