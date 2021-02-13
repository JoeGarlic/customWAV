use std::io::{self, stdin, Write};
use std::path::PathBuf;

use clap::Clap;

#[derive(Clap, Debug)]
#[clap(
    author = "Garlic <github@JoeGarlic>, Dabrick <github@dmaahs2017>",
    version = "0.0.3",
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
        io::stdout().flush().unwrap();
        if let Err(e) = stdin.read_line(&mut directory) {
            eprintln!(
                "Failed to read directory input. The following error was produced:\n{}",
                e
            );
            io::stdout().flush().unwrap();
            std::process::exit(1);
        }

        print!("Samples to join (extension optional): ");
        io::stdout().flush().unwrap();
        if let Err(e) = stdin.read_line(&mut samples) {
            eprintln!(
                "Failed to read samples input. The following error was produced:\n{}",
                e
            );
            io::stdout().flush().unwrap();
            std::process::exit(1);
        }
        let samples: Vec<&str> = samples.split_whitespace().collect();

        print!("Output file name (extension optional): ");
        io::stdout().flush().unwrap();
        if let Err(e) = stdin.read_line(&mut output) {
            eprintln!(
                "Failed to read output file input. The following error was produced:\n{}",
                e
            );
            io::stdout().flush().unwrap();
            std::process::exit(1);
        }

        let res = sound_bored::join_samples_to_new_wav(&output, &directory, &samples, 44100);
        handle_error(res);
    } else {
        let res = sound_bored::join_samples_to_new_wav(
            &args.output,
            &args.sample_directory,
            &args.samples,
            44100,
        );
        handle_error(res);
    }
}

fn handle_error(e: sound_bored::SBResult) {
    match e {
        Ok(()) => (),
        Err(e) => match e {
            sound_bored::Error::NoSamples => {
                eprintln!("No samples were input");
                io::stdout().flush().unwrap();
                std::process::exit(1);
            }
            sound_bored::Error::DirectoryNotFound(d) => {
                eprintln!(
                    "Directory not found: {}\nCurrent Directory: {}",
                    d.to_string_lossy(),
                    std::env::current_dir().unwrap().to_string_lossy()
                );
                io::stdout().flush().unwrap();
                std::process::exit(1);
            }
            sound_bored::Error::HoundErr(e) => {
                eprintln!("Encoding Error: {}", e);
                io::stdout().flush().unwrap();
                std::process::exit(1);
            }
            sound_bored::Error::FileNotFound(dir, name) => {
                eprintln!(
                    "Could not find mp3 or wav file named {} in directory {}",
                    name,
                    dir.to_string_lossy()
                );
                io::stdout().flush().unwrap();
                std::process::exit(1);
            }
            sound_bored::Error::CreakErr(e) => {
                eprintln!("Decoding Error: {}", e);
                io::stdout().flush().unwrap();
                std::process::exit(1);
            }
            sound_bored::Error::ChannelCoversionError(source, target) => {
                eprintln!(
                    "Failed converting channel data from {} channels to {} channels",
                    source.to_string(),
                    target.to_string()
                );
                io::stdout().flush().unwrap();
                std::process::exit(1);
            }
        },
    }
}
