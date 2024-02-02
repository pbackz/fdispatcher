use std::path::PathBuf;
use clap::{Command, Arg};
use walkdir::WalkDir;
use std::fs;

const MP4:           &str = "mp4";
const JPG:           &str = "jpg";
const PNG:           &str = "png";
const MKV:           &str = "mkv";
const WAV:           &str = "wav";
const PDF:           &str = "pdf";
const ASM:           &str = "asm";
const BLEND:         &str = "blend";
const BMP:           &str = "bmp";
const BZ2:           &str = "bz2";
const C:             &str = "c";
const CLASS:         &str = "class";
const CONF:          &str = "conf";
const CFG:           &str = "cfg";
const CPP:           &str = "cpp";
const CC:            &str = "cc";
const CSS:           &str = "css";
const CSV:           &str = "csv";
const DB:            &str = "db";
const DEB:           &str = "deb";
const DESKTOP:       &str = "desktop";
const DIFF:          &str = "diff";
const DTD:           &str = "dtd";
const GIF:           &str = "gif";
const GZ:            &str = "gz";
const H:             &str = "h";
const HTML:          &str = "html";
const HTM:           &str = "htm";
const JAR:           &str = "jar";
const JAVA:          &str = "java";
const KWD:           &str = "kwd";
const KSP:           &str = "ksp";
const KSS:           &str = "kss";
const LOG:           &str = "log";
const M3U:           &str = "m3u";
const M4A:           &str = "m4a";
const M4P:           &str = "m4p";
const MD5:           &str = "md5";
const MD5SUMS:       &str = "md5sums";
const MO:            &str = "mo";
const MPGOR:         &str = "mpgor";
const MPEG:          &str = "mpeg";
const OGG:           &str = "ogg";
const PATCH:         &str = "patch";
const PHP:           &str = "php";
const PHPS:          &str = "phps";
const PHTML:         &str = "phtml";
const PL:            &str = "pl";
const PLS:           &str = "pls";
const POV:           &str = "pov";
const PROPERTIES:    &str = "properties";
const PS:            &str = "ps";
const PY:            &str = "py";
const PYO:           &str = "pyo";
const PYC:           &str = "pyc";
const RDF:           &str = "rdf";
const RS:            &str = "rs";
const GO:            &str = "go";
const RPM:           &str = "rpm";
const RTF:           &str = "rtf";
const SH:            &str = "sh";
const SO:            &str = "so";
const TAR:           &str = "tar";
const TGZ:           &str = "tgz";
const TTF:           &str = "ttf";
const TXT:           &str = "txt";
const XBEL:          &str = "xbel";
const XSD:           &str = "xsd";
const XML:           &str = "xml";
const XSL:           &str = "xsl";
const XPM:           &str = "xpm";
const ZIP:           &str = "zip";

fn main() {
    let cmd = Command::new("fd")
        .about("A File dispatcher based on extension")
        .bin_name("fd")
        .subcommand_required(true)
        .subcommands(vec![
            create_subcommand(MP4),
            create_subcommand(JPG),
            create_subcommand(PNG),
            create_subcommand(MKV),
            create_subcommand(WAV),
            create_subcommand(PDF),
            create_subcommand(ASM),
            create_subcommand(BLEND),
            create_subcommand(BMP),
            create_subcommand(BZ2),
            create_subcommand(C),
            create_subcommand(CLASS),
            create_subcommand(CONF),
            create_subcommand(CFG),
            create_subcommand(CPP),
            create_subcommand(CC),
            create_subcommand(CSS),
            create_subcommand(CSV),
            create_subcommand(DB),
            create_subcommand(DEB),
            create_subcommand(DESKTOP),
            create_subcommand(DIFF),
            create_subcommand(DTD),
            create_subcommand(GIF),
            create_subcommand(GZ),
            create_subcommand(H),
            create_subcommand(HTML),
            create_subcommand(HTM),            
            create_subcommand(JAR),
            create_subcommand(JAVA),
            create_subcommand(KWD),
            create_subcommand(KSP),
            create_subcommand(KSS),
            create_subcommand(LOG),
            create_subcommand(M3U),
            create_subcommand(M4A),
            create_subcommand(M4P),
            create_subcommand(MD5),
            create_subcommand(MD5SUMS),
            create_subcommand(MO),
            create_subcommand(MPGOR),
            create_subcommand(MPEG),
            create_subcommand(OGG),
            create_subcommand(PATCH),
            create_subcommand(PHP),
            create_subcommand(PHPS),
            create_subcommand(PHTML),
            create_subcommand(PL),
            create_subcommand(PLS),
            create_subcommand(POV),
            create_subcommand(PROPERTIES),
            create_subcommand(PS),
            create_subcommand(PY),
            create_subcommand(PYO),
            create_subcommand(PYC),
            create_subcommand(RDF),
            create_subcommand(RS),
            create_subcommand(GO),
            create_subcommand(RPM),
            create_subcommand(RTF),
            create_subcommand(SH),
            create_subcommand(SO),
            create_subcommand(TAR),
            create_subcommand(TGZ),
            create_subcommand(TTF),
            create_subcommand(TXT),
            create_subcommand(XBEL),
            create_subcommand(XSD),
            create_subcommand(XML),
            create_subcommand(XSL),
            create_subcommand(XPM),
            create_subcommand(ZIP),
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
