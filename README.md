# sound-bored
This CLI app is for making audio memes with your friends

## Examle usage
`sound-bored -h` - to show the help menu

`sound-bored sound1 sound2 sound3 <... soundN>` to create a `custom.wav` file that is the combination of all the sound files listed

### Other flags
`-d <directory>` use this flag to specify the directory of your sound samples, if left out it will search the current directory

`-o <name>` use this flag to specify an output name, if left out the output name will be `custom.wav`

`-i` use this flag for an interactive prompt, this flag overrides all other flags

## But why?
My friends record audio samples of words/sounds to create sentences and play it back through discord with a hotkey. It's for memes.

## Installation
Just download one of the latest version binaries for your platform on the [releases page](https://github.com/dmaahs2017/sound-bored/releases)

Then place it in the directory you would like to use it, or add it to your `$PATH`. (Yay for static binaries)

Or clone this repository and compile from source for the most up to date version using `cargo build --release`, see [installing rust](https://www.rust-lang.org/tools/install).
