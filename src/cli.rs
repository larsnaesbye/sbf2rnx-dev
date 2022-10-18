use clap::Command;
use clap::{Arg, ArgMatches};

pub fn matches() -> ArgMatches {
    //! Handle all command line parameters

    let matches = Command::new("sbf2rnx-dev")
        .version("0.1.0")
        .author("Lars Næsbye Christensen <lars@naesbye.dk>")
        .about("Preliminary work for a Septentrio SBF to RINEX converter")
        .arg(
            Arg::new("filepath")
                .short('f')
                .long("filepath")
                .required(true)
                .help("path to SBF file"),
        )
        .arg(Arg::new("interval").short('i').long("interval"))
        .arg(Arg::new("output").short('o').long("output"))
        .arg_required_else_help(true)
        .get_matches();
    return matches;
}
