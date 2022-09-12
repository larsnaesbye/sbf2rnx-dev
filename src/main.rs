//! This application converts Septentrio binary files (SBF)
//! into receiver independent (RINEX) files.  
//! Specification is found on page 244 on https://www.septentrio.com/system/files/support/asterx_sb_firmware_v4.8.4_reference_guide.pdf
mod sbf;

use clap::Command;
use clap::{Arg, ArgMatches};
use rinex::Rinex;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("std::io error")]
    IoError(#[from] std::io::Error),
}

pub fn main() -> Result<(), Error> {
    let matches = matches();
    let filepath: Option<Vec<&str>> = match matches.is_present("filepath") {
        true => Some(matches.value_of("filepath").unwrap().split(",").collect()),
        false => None,
    };

    let rinexrec = sbf2rnxrec(filepath);
    write_rnx_file(rinexrec);
    return Ok(());
}
//! Handle all command line parameters
fn matches() -> ArgMatches {
    let matches = Command::new("sbf2rnx-dev")
        .version("0.1.0")
        .author("Lars Næsbye Christensen <lars@naesbye.dk>")
        .about("Preliminary work for a Septentrio SBF to RINEX converter")
        .arg(
            Arg::new("filepath")
                .short('f')
                .long("filepath")
                .required(true),
        )
        .arg(Arg::new("interval").short('i').long("interval"))
        .arg(Arg::new("output").short('o').long("output"))
        .arg_required_else_help(true)
        .get_matches();
    return matches;
}

fn sbf2rnxrec(_filepath: Option<Vec<&str>>) -> Rinex {
    // TODO: build RINEX records and output them as files
    return Rinex::default();
}

fn write_rnx_file(rinexrec: Rinex) {
    rinexrec.to_file("").expect("TODO: panic message");
}
