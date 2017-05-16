use std::fs::{File, OpenOptions};
use std::io::Write;

extern crate clap;
use clap::{App, Arg};

extern crate chariot_slp;

fn main() {
    let matches = App::new("slpcheck")
        .version("0.1.0")
        .author("Taryn Hill <taryn@phrohdoh.com>")
        .about("Test SLPs image files for invalid commands")
        .arg(Arg::with_name("slp-path")
            .long("slp-path")
            .value_name("/path/to/your.slp")
            .help("Filepath to the SLP to check")
            .required(true)
            .takes_value(true))
        .get_matches();

    let slp_path = matches.value_of("slp-path").unwrap();
    let slp = chariot_slp::SlpFile::read_from_file(slp_path).expect(&format!("Failed to read {}", slp_path));

    println!("file_version: {:?}", slp.header.file_version);
    println!("shape_count: {}", slp.header.shape_count);
    println!("comment: {:?}", slp.header.comment);
}