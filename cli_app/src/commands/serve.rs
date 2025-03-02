use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use axum::{Router, ServiceExt};
use clap::{value_parser, Arg, ArgMatches, Command};
use crate::settings::Settings;

pub const COMMAND_NAME: &str = "serve";

pub fn configure() -> Command {
    Command::new(COMMAND_NAME).about("Starts HTTP Server").arg(
        Arg::new("port")
            .short('p')
            .long("port")
            .value_name("PORT")
            .help("Specify the HTTP server port")
            .default_value("8080")
            .value_parser(value_parser!(u16)),
    )
}

pub fn handle(matches: &ArgMatches,settings: &Settings) -> anyhow::Result<()> {
    let port: u16 = *matches.get_one("port").unwrap_or(&8080);

    start_tokio(port,settings)?;
    println!("TBD: start the webserver on port {}", port);

    Ok(())
}

fn start_tokio(port:u16, settings: &Settings) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async move {
            // TBD ...
            let  addr = SocketAddr::new(
                IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
                port
            );

            let router = Router::new();
            let listener = tokio::net::TcpListener::bind(addr).await?;
            axum::serve(listener,router.into_make_service()).await?;

            Ok::<(), anyhow::Error>(())
        })?;

    Ok(())
}