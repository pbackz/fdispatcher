use std::path::PathBuf;
use clap::Command;
use walkdir::WalkDir;
use std::fs;

fn main() {
    let cmd = Command::new("fdispatcher")
    .about("A File dispatcher based on extension")
        .bin_name("fdispatcher")
        .subcommand_required(true)
        .subcommand(
            clap::command!("mp4")
            .about("Perform .mp4 file extension move")
            .arg(
                clap::arg!(--"source-dir" <PATH>)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .short('s')
                    .help("The source directory where iterate recursively"),
            )
            .arg(
                clap::arg!(--"target-dir" <PATH>)
                    .short('t')
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("The source directory where iterate recursively"),
            ),
        )
        .subcommand(
            clap::command!("jpg")
            .about("Perform .jpg file extension move")
            .arg(
                clap::arg!(--"source-dir" <PATH>)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("The source directory where iterate recursively"),
            )
            .arg(
                clap::arg!(--"source-dir" <PATH>)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .short('s')
                    .help("The source directory where iterate recursively"),
            )
            .arg(
                clap::arg!(--"target-dir" <PATH>)
                    .short('t')
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("The source directory where iterate recursively"),
            ),
        )
        .subcommand(
            clap::command!("png")
            .about("Perform .png file extension move")
            .arg(
                clap::arg!(--"source-dir" <PATH>)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .short('s')
                    .help("The source directory where iterate recursively"),
            )
            .arg(
                clap::arg!(--"target-dir" <PATH>)
                    .short('t')
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("The source directory where iterate recursively"),
            ),
        )
        .subcommand(
            clap::command!("mkv")
            .about("Perform .mkv file extension move")
            .arg(
                clap::arg!(--"source-dir" <PATH>)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .short('s')
                    .help("The source directory where iterate recursively"),
            )
            .arg(
                clap::arg!(--"target-dir" <PATH>)
                    .short('t')
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("The source directory where iterate recursively"),
            ),
        )
        .subcommand(
            clap::command!("wav")
            .about("Perform .wav file extension move")
            .arg(
                clap::arg!(--"source-dir" <PATH>)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .short('s')
                    .help("The source directory where iterate recursively"),
            )
            .arg(
                clap::arg!(--"target-dir" <PATH>)
                    .short('t')
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("The source directory where iterate recursively"),
            ),
        )
        .subcommand(
            clap::command!("pdf")
            .about("Perform .pdf file extension move")
            .arg(
                clap::arg!(--"source-dir" <PATH>)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .short('s')
                    .help("The source directory where iterate recursively"),
            )
            .arg(
                clap::arg!(--"target-dir" <PATH>)
                    .short('t')
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("The source directory where iterate recursively"),
            ),
        );
    let matches = cmd.get_matches();
    let matches = match matches.subcommand() {
        Some(("mp4", matches)) => matches,
        Some(("wav", matches)) => matches,
        Some(("jpg", matches)) => matches,
        Some(("png", matches)) => matches,
        Some(("pdf", matches)) => matches,
        Some(("mkv", matches)) => matches,
        _ => unreachable!("clap should ensure we don't get here"),
    };

    let source_dir = matches.get_one::<std::path::PathBuf>("source-dir");
    let target_dir = matches.get_one::<std::path::PathBuf>("target-dir");
    //debug
    //println!("{source_dir:?}");
    //println!("{target_dir:?}");
    
    if Some(("mp4", matches)).is_some() {
        move_files(source_dir.unwrap(), target_dir.unwrap(), "mp4".to_string())
    }
    if Some(("wav", matches)).is_some() {
        move_files(source_dir.unwrap(), target_dir.unwrap(), "wav".to_string())
    }
    if Some(("jpg", matches)).is_some() {
        move_files(source_dir.unwrap(), target_dir.unwrap(), "jpg".to_string())
    }
    if Some(("png", matches)).is_some() {
        move_files(source_dir.unwrap(), target_dir.unwrap(), "png".to_string())
    }
    if Some(("pdf", matches)).is_some() {
        move_files(source_dir.unwrap(), target_dir.unwrap(), "pdf".to_string())
    }
    if Some(("mkv", matches)).is_some() {
        move_files(source_dir.unwrap(), target_dir.unwrap(), "mkv".to_string())
    }
}

fn move_files(sourcedir: &PathBuf, targetdir: &PathBuf, extension: String) {
    for entry in WalkDir::new(sourcedir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some(&extension) {
            let destination = targetdir.join(path.file_name().unwrap());
            fs::rename(path, &destination).expect("Error moving file");
            println!("Moved {:?} to {:?}", path, destination);
        }
    }
}