//! This application converts Septentrio binary files
//! into receiver independent (RINEX) files.  

use clap::load_yaml;
use clap::App;
use rinex::Rinex;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("std::io error")]
    IoError(#[from] std::io::Error),
}

pub fn main() -> Result<(), Error> {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml); // TODO: replace deprecated YAML call with arg-builder
    let matches = app.get_matches();

    let filepaths: Option<Vec<&str>> = match matches.is_present("filepath") {
        true => Some(matches.value_of("filepath").unwrap().split(",").collect()),
        false => None,
    };

    let mut rinexrec = sbf2rnxrec(filepaths);
    write_rnx_file(rinexrec);
    return Ok(());
}

fn sbf2rnxrec(filepaths: Option<Vec<&str>>) -> Rinex {
    // TODO: build RINEX records and output them as files
    return Rinex::default();
}

fn write_rnx_file(rinexrec: Rinex) {}
