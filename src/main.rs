use std::fs;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};

use glob::glob;
use sanitize_filename::sanitize_with_options;

macro_rules! find {
    ( $( $ext:expr ),+ ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                let pattern = concat!("**/*.", $ext);
                for entry in glob(pattern).unwrap() {
                    temp_vec.push(entry.unwrap());
                }
            )+
            temp_vec
        }
    };
}

const OPTIONS: sanitize_filename::Options = sanitize_filename::Options {
    truncate: true,
    windows: true,
    replacement: "_",
};

fn main() {
    let mut args = std::env::args();
    if args.len() != 2 {
        eprintln!("USAGE: muscp <FOLDER>");
        exit(1);
    }

    let mut target_dir = PathBuf::from(args.nth(1).unwrap());
    target_dir.push(std::env::current_dir().unwrap().file_name().unwrap());

    let lossy: Vec<PathBuf> = find!("aac", "m4a", "mp3", "ogg", "oga", "opus", "wma");
    let lossless: Vec<PathBuf> = find!("aiff", "alac", "ape", "flac", "raw", "wav");

    for file in lossy {
        cp(&file, &target_dir);
    }
    for file in lossless {
        let mp3 = to_mp3(&file);
        mv(&mp3, &target_dir);
    }
}

fn sanitize_path(p: &Path) -> PathBuf {
    let components: Vec<_> = p
        .components()
        .map(|c| {
            if c.as_os_str().to_string_lossy() == "/" {
                "".to_string()
            } else {
                sanitize_with_options(c.as_os_str().to_string_lossy(), OPTIONS)
            }
        })
        .collect();

    PathBuf::from(components.join("/"))
}

fn cp(file: &PathBuf, target: &Path) {
    let dest = sanitize_path(&target.join(&file));
    eprint!(
        "Copying {:#?} to {}... ",
        file.file_stem().unwrap(),
        dest.parent().unwrap().display()
    );
    fs::create_dir_all(&dest.parent().unwrap()).expect("Failed to create target directory");
    fs::copy(file, &dest).expect("Failed to copy file");
    eprintln!("OK");
}

fn mv(file: &PathBuf, target: &Path) {
    cp(file, target);
    fs::remove_file(&file).expect("Failed to remove file");
}

fn to_mp3(file: &Path) -> PathBuf {
    eprint!("Converting {:#?} to mp3... ", file.file_stem().unwrap());
    let input = file.display().to_string();
    let output = file.with_extension("mp3").display().to_string();
    Command::new("ffmpeg")
        .args(["-i", &input, &output])
        .output()
        .expect("Failed to convert file");
    eprintln!("OK");
    PathBuf::from(output)
}
