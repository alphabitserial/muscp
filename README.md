muscp
=====
A small utility for taking the hassle out of copying music to your mobile device.

## Usage example
```
~/Music/Lexie Liu 
❯ muscp ~/target/folder
Copying "1. Like a Mercedes" to /home/user/target/folder/Lexie Liu/2029... OK
Copying "10. Dripping Sauce" to /home/user/target/folder/Lexie Liu/2029... OK
Copying "2. Mulan" to /home/user/target/folder/Lexie Liu/2029... OK
Copying "3. Sleep Away" to /home/user/target/folder/Lexie Liu/2029... OK
Copying "4. Watch Me" to /home/user/target/folder/Lexie Liu/2029... OK
    -snip-
Converting "01 ALGTR" to mp3... OK
Copying "01 ALGTR" to /home/user/target/folder/Lexie Liu/GONE GOLD... OK
Converting "02 SHADOW" to mp3... OK
Copying "02 SHADOW" to /home/user/target/folder/Lexie Liu/GONE GOLD... OK
Converting "03 CAROUSEL" to mp3... OK
Copying "03 CAROUSEL" to /home/user/target/folder/Lexie Liu/GONE GOLD... OK
    -snip-
~/Music/Lexie Liu took 24s 
❯
```

## What it does
Recurse through the current directory, copying music files to a target folder, while...
* Sanitizing filenames for use on e.g. Android or Windows
* Converting hi-res audio formats like flac to mp3

Hi-res formats are converted next to the original, then moved. All other files are simply copied.

## Dependencies
* [Rust](https://rustup.rs/) is required to build *muscp* from source.
* [ffmpeg](https://ffmpeg.org/) needs to be in your PATH to convert hi-res audio to mp3. (How to set PATH: [Windows](https://stackoverflow.com/a/44272417), [Linux](https://unix.stackexchange.com/a/26059), [macOS](https://unix.stackexchange.com/a/111557))

## How to install
Download or clone this repo, enter its folder in your terminal or command prompt, and type `cargo install --path .`