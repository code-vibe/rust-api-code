use clap::{Arg, Command};
use cli_app::{commands, settings};
pub fn main() -> anyhow::Result<()> {

    let mut command = Command::new("Sample CLI application")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .help("Configuration file location")
                .default_value("config.json"),
        );
    command = commands::configure(command);


    let matches = command.get_matches();
    let config_location = matches
        .get_one("config")
        .map(|s: &String| s.as_str())
        .unwrap_or("");

    commands::handle(&matches)?;


    let settings = settings::Settings::new(config_location, "APP")?;
    println!(
        "db url {}",
        settings.database.url.unwrap_or("missing database url".to_string())
    );

    println!(
        "log level: {}",
        settings.logging.log_level.unwrap_or("info".to_string())
    );
    Ok(())
}

