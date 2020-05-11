extern crate clap;
#[cfg(feature = "dotenv")]
extern crate dotenv;

use clap::{Arg, App, crate_name, crate_version, crate_authors, crate_description};

fn main() {
    #[cfg(feature = "dotenv")] {
        if let Some(e) = dotenv::dotenv().err() {
            eprintln!("Cannot load .env file: {}", e);
        }
    }

    let matches = App::new(crate_name!())
    .version(crate_version!())
    .author(crate_authors!())
    .about(crate_description!())
    .arg(Arg::with_name("id-scope")
        .env("BUSY_LIGHT_ID_SCOPE")
        .long("id-scope")
        .required(true)
        .help("The ID scope provided by Azure"))
    .arg(Arg::with_name("device-id")
        .env("BUSY_LIGHT_DEVICE_ID")
        .long("device-id")
        .required(true)
        .help("The device ID provided by Azure"))
    .arg(Arg::with_name("primary-key")
        .env("BUSY_LIGHT_PRIMARY_KEY")
        .long("primary-key")
        .required(true)
        .help("The primary key provided by Azure"))
    .arg(Arg::with_name("provisioning-host")
        .env("BUSY_LIGHT_PROVISIONING_HOST")
        .long("provisioning-host")
        .required(true)
        .takes_value(true)
        .default_value("global.azure-devices-provisioning.net")
        .help("The provisioning host used by Azure"))
    .arg(Arg::with_name("v")
      .short("v")
      .multiple(true)
      .help("Sets the level of verbosity"))
    .get_matches();
}
