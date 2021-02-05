use sound_bored;

#[test]
fn the_fuck_works() {
    let res =
        sound_bored::join_samples_to_new_wav("the_fuck", "tests/samples", &vec!["the", "fuck"]);
    assert!(res.is_ok());
}

#[test]
fn and_fuck_the() {
    let res = sound_bored::join_samples_to_new_wav(
        "and_fuck_the",
        "tests/samples",
        &vec!["and", "fuck", "the"],
    );
    assert!(res.is_ok());
}

#[test]
fn invalid_directory_returns_error() {
    let res = sound_bored::join_samples_to_new_wav("test", "does/not/exist", &["hi"]);
    match res {
        Ok(()) => panic!("Should have returned an Error"),
        Err(e) => match e {
            sound_bored::Error::HoundError(_) => panic!("Incorrect error type"),
            sound_bored::Error::NoSamples => panic!("Incorrect Error Type"),
            sound_bored::Error::DirNotFound(t) => assert_eq!(t, std::path::PathBuf::from("does/not/exist")),
        },
    }
}

#[test]
fn no_samples_input_returns_error() {
    let empty: &[&str] = &[];
    let res = sound_bored::join_samples_to_new_wav("test", "tests/samples", empty);
    match res {
        Ok(()) => panic!("Should have returned an Error"),
        Err(e) => match e {
            sound_bored::Error::HoundError(_) => panic!("Incorrect error type"),
            sound_bored::Error::DirNotFound(_) => panic!("Incorrect Error Type"),
            sound_bored::Error::NoSamples => (),
        },
    }
}

#[test]
fn generated_file_is_always_the_same() {
    let new_audio_filename = "tests/samples/cross_check_hasher.wav";
    let known_good_filename = "tests/samples/hasher_base.wav";

    sound_bored::join_samples_to_new_wav("cross_check_hasher", "tests/samples/", &vec!["fuck"]).unwrap();

    let new = std::fs::read(new_audio_filename).unwrap();
    let good = std::fs::read(known_good_filename).unwrap();

    let new_hash = md5::compute(new);
    let good_hash = md5::compute(good);

    assert_eq!(new_hash, good_hash);
}
