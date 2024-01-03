//! This application converts Septentrio binary files (SBF)
//! into receiver independent (RINEX) files.  
//! Specification is found on page 244 on https://www.septentrio.com/system/files/support/asterx_sb_firmware_v4.8.4_reference_guide.pdf
mod sbf;

use rinex::Rinex;
use std::path::Path;
use thiserror::Error;
use clap::{arg, Command};

#[derive(Debug, Error)]
pub enum Error {
    #[error("std::io error")]
    IoError(#[from] std::io::Error),
}

pub fn main() -> Result<(), Error> {
    let matches = Command::new("sbf2rnx")
        .version("0.1")
        .author("Lars Næsbye Christensen <lars@naesbye.dk>")
        .about("Does awesome things")
        .arg(arg!(--i <VALUE>).required(true))
        .get_matches();
    let filepath: String = matches.get_one::<String>("i").unwrap().to_string();
    let rinexrec = sbf2rnxrec(filepath);
    // write_rnx_file(rinexrec);
    Ok(())
}

fn sbf2rnxrec(filepath: String) -> Rinex {
    //! Build RINEX records and output them as files
    // For now we read the entire file as bytes before conversion - this uses more memory!

    let rrecord: Rinex = Rinex::default();
    match std::fs::read(Path::new(&filepath).as_os_str()) {
        Ok(bytes) => {
            for byte in bytes {
                eprintln!("Read byte {}", byte);
            }
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                eprintln!("Please check path {}", &filepath);
                panic!("{}", e)
            }
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                eprintln!("Please run again with appropriate permissions.");
                panic!("{}", e)
            }
            panic!("{}", e);
        }
    }

    rrecord
}

fn write_rnx_file(rinexrec: Rinex) {
    rinexrec.to_file("test.rnx").expect("Error: RINEX writeout failed.");
}
