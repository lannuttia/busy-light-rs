extern crate clap;
extern crate simplelog;

#[cfg(feature = "dotenv")]
extern crate dotenv;

fn main() {
    #[cfg(feature = "dotenv")]
    {
        if let Some(e) = dotenv::dotenv().err() {
            eprintln!("Cannot load .env file: {}", e);
        }
    }

    let matches = clap::App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(
            clap::Arg::with_name("id-scope")
                .env("BUSY_LIGHT_ID_SCOPE")
                .long("id-scope")
                .required(true)
                .help("The ID scope provided by Azure"),
        )
        .arg(
            clap::Arg::with_name("device-id")
                .env("BUSY_LIGHT_DEVICE_ID")
                .long("device-id")
                .required(true)
                .help("The device ID provided by Azure"),
        )
        .arg(
            clap::Arg::with_name("primary-key")
                .env("BUSY_LIGHT_PRIMARY_KEY")
                .long("primary-key")
                .required(true)
                .help("The primary key provided by Azure"),
        )
        .arg(
            clap::Arg::with_name("provisioning-host")
                .env("BUSY_LIGHT_PROVISIONING_HOST")
                .long("provisioning-host")
                .takes_value(true)
                .default_value("global.azure-devices-provisioning.net")
                .help("The provisioning host used by Azure"),
        )
        .arg(
            clap::Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .get_matches();

    let level_filter = match matches.occurrences_of("v") {
        0 => simplelog::LevelFilter::Warn,
        1 => simplelog::LevelFilter::Info,
        2 => simplelog::LevelFilter::Debug,
        3 | _ => simplelog::LevelFilter::Trace,
    };

    simplelog::CombinedLogger::init(vec![simplelog::TermLogger::new(
        level_filter,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
    )
    .unwrap()])
    .unwrap();
}
