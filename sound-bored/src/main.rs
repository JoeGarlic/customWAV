use sound_bored::join_samples_to_new_wav;
use std::io::stdin;
fn main() {
    println!("Enter sentence: ");
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("Input failed");

    let words: Vec<&str> = buf.split_whitespace().collect();

    join_samples_to_new_wav("_custom", "words", &words).expect("new file failed to generate");
}
