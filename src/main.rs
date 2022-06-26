use std::fs::File;
use std::io::prelude::*;
use xdg;
use clap::Parser;


/// Unofficial Logitech Options for Linux 
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Name of the person to greet
    #[clap(short, long, value_parser)]
    name: String,

    /// Number of times to greet
    #[clap(short, long, value_parser, default_value_t = 1)]
    count: u8,
}


fn main() -> std::io::Result<()> {
    let app_name = "logi";
    let config_file_name = "config.toml";
    let xdg_dirs = xdg::BaseDirectories::with_prefix(app_name).unwrap();
    let config_files = xdg_dirs.list_config_files(config_file_name);

    let config_file = if config_files.is_empty() {
        xdg_dirs.place_config_file(config_file_name).expect("cannot create configuration directory")
    } else {
        config_files.first().expect("error here").to_owned()
    };

    let mut config_file = File::create(config_file).expect("error here");
    config_file.write_all(b"Hello, world!")?;

    let args = Cli::parse();

    Ok(())
}
