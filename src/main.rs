use std::io::stdin;
use std::path::PathBuf;

use clap::Clap;
use sound_bored::join_samples_to_new_wav;

//mod args;

#[derive(Clap, Debug)]
#[clap(
    author = "Garlic <github@JoeGarlic>, Dabrick <github@dmaahs2017>",
    version = "0.0.1",
    about = "sound-bored is a meme, that helps you make audio memes on demand"
)]
struct Args {
    samples: Vec<String>,
    #[clap(short = 'd', long, default_value = ".", parse(from_os_str))]
    sample_directory: PathBuf,
    #[clap(short = 'o', long, default_value = "custom")]
    output: String,
    #[clap(short, long)]
    interactive: bool,
}

fn main() {
    let args: Args = Args::parse();

    if args.interactive {
        let mut directory = String::new();
        let mut samples = String::new();
        let mut output = String::new();
        let stdin = stdin();

        print!("Samples Directory: ");
        if let Err(e) = stdin.read_line(&mut directory) {
            eprintln!(
                "Failed to read directory input. The following error was produced:\n{}",
                e
            );
            std::process::exit(1);
        }

        print!("Samples to join (extension optional): ");
        if let Err(e) = stdin.read_line(&mut samples) {
            eprintln!(
                "Failed to read samples input. The following error was produced:\n{}",
                e
            );
            std::process::exit(1);
        }
        let samples: Vec<&str> = samples.split_whitespace().collect();

        print!("Output file name (extension optional): ");
        if let Err(e) = stdin.read_line(&mut output) {
            eprintln!(
                "Failed to read output file input. The following error was produced:\n{}",
                e
            );
            std::process::exit(1);
        }

        let res = join_samples_to_new_wav(&output, &directory, &samples);
        handle_error(res);
    } else {
        let res = join_samples_to_new_wav(&args.output, &args.sample_directory, &args.samples);
        handle_error(res);
    }
}

fn handle_error(e: sound_bored::SBResult) {
    match e {
        Ok(()) => (),
        Err(e) => {
            match e {
                sound_bored::Error::NoSamples => {
                    eprintln!("No samples were input");
                    std::process::exit(1);
                }
                sound_bored::Error::DirNotFound(d) => {
                    eprintln!("Directory not found: {}", d.to_str().unwrap_or(""));
                    std::process::exit(1);
                }
                sound_bored::Error::HoundError(e) => {
                    eprintln!("Encoding/Decoding Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}
