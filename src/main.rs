use clap::load_yaml;
use clap::App;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("std::io error")]
    IoError(#[from] std::io::Error),
}

pub fn main() -> Result<(), Error> {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml);
    let matches = app.get_matches();
    return Ok(());
}

fn sbfrec2rnxrec() {
    println!("nada yet")
}
