use std::path::PathBuf;
use clap::{Command, Arg};
use walkdir::WalkDir;
use std::fs;

const MP4: &str = "mp4";
const JPG: &str = "jpg";
const PNG: &str = "png";
const MKV: &str = "mkv";
const WAV: &str = "wav";
const PDF: &str = "pdf";

fn main() {
    let cmd = Command::new("fdispatcher")
        .about("A File dispatcher based on extension")
        .bin_name("fdispatcher")
        .subcommand_required(true)
        .subcommands(vec![
            create_subcommand(MP4),
            create_subcommand(JPG),
            create_subcommand(PNG),
            create_subcommand(MKV),
            create_subcommand(WAV),
            create_subcommand(PDF),
        ]);

    let matches = cmd.get_matches();

    if let Some((extension, sub_matches)) = matches.subcommand() {
        let source_dir = sub_matches.get_one::<PathBuf>("source-dir").expect("Required source directory");
        let target_dir = sub_matches.get_one::<PathBuf>("target-dir").expect("Required target directory");
        move_files(source_dir, target_dir, extension);
    }
}

fn create_subcommand(cmd_name: &'static str) -> Command {
    Command::new(cmd_name)
        .about(&format!("Perform .{} file extension move", cmd_name))
        .arg(
            Arg::new("source-dir")
                .long("source-dir")
                .value_parser(clap::value_parser!(PathBuf))
                .short('s')
                .help("The source directory where files will be processed recursively")
                .required(true),
        )
        .arg(
            Arg::new("target-dir")
                .long("target-dir")
                .value_parser(clap::value_parser!(PathBuf))
                .short('t')
                .help("The target directory where files will be moved")
                .required(true),
        )
}

fn move_files(sourcedir: &PathBuf, targetdir: &PathBuf, extension: &str) {
    for entry in WalkDir::new(sourcedir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some(extension) {
            let destination = targetdir.join(path.file_name().expect("Failed to get file name"));
            if let Err(e) = fs::rename(path, &destination) {
                eprintln!("Error moving file {:?} to {:?}: {}", path, destination, e);
            } else {
                println!("Moved {:?} to {:?}", path, destination);
            }
        }
    }
}
