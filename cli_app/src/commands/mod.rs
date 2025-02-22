use clap::{ArgMatches, Command};

pub fn configure(command: Command) -> Command {
    command.subcommand(Command::new("hello").about("Hello World!"))
}

pub fn handle(matches: &ArgMatches) -> anyhow::Result<()> {
    if let Some((cmd, _matches)) = matches.subcommand() {
        match cmd {
            "hello" => { println!("Hello world!"); },
            &_ => {}
        }
    }

    Ok(())
}