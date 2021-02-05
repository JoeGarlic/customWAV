use sound_bored;

#[test]
fn the_fuck_works() {
    let res = sound_bored::join_samples_to_new_wav("the_fuck", "tests/samples", &vec!["the", "fuck"]);
    assert!(res.is_ok());
}

#[test]
fn and_fuck_the() {
    let res = sound_bored::join_samples_to_new_wav("and_fuck_the", "tests/samples", &vec!["and", "fuck", "the"]);
    assert!(res.is_ok());
}
