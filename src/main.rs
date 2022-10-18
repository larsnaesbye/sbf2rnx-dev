//! This application converts Septentrio binary files (SBF)
//! into receiver independent (RINEX) files.  
//! Specification is found on page 244 on https://www.septentrio.com/system/files/support/asterx_sb_firmware_v4.8.4_reference_guide.pdf
mod sbf;
mod cli;

use cli::matches;
use rinex::Rinex;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("std::io error")]
    IoError(#[from] std::io::Error),
}

pub fn main() -> Result<(), Error> {
    let matches = matches();
    //let filepath: Option<Vec<&str>> = match matches.is_present("filepath") {
    //    true => Some(matches.value_of("filepath").unwrap().split(",").collect()),
    //    false => None,
    //};
    // get the file data
    //let rinexrec = sbf2rnxrec(filepath.unwrap());
    //write_rnx_file(rinexrec);
    return Ok(());
}

fn sbf2rnxrec(filepath: Vec<&str>) -> Rinex {
    //! Build RINEX records and output them as files
    // For now we read the entire file as bytes before conversion - this uses more memory!
    //match std::fs::read(Path::new(&filepath.as_ptr())) {
    //    Ok(bytes) => {
    //        eprintln!("{}", bytes.len().to_string()) // For testing, just print the length of the file
    //    }
    //    Err(e) => {
    //        if e.kind() == std::io::ErrorKind::PermissionDenied {
    //            eprintln!("Please run again with appropriate permissions.");
    //            panic!("{}", e)
    //        }
    //        panic!("{}", e);
    //    }
    //}

    return Rinex::default();
}

fn write_rnx_file(rinexrec: Rinex) {
    rinexrec.to_file("").expect("TODO: panic message");
}
