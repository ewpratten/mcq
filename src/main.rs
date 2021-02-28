#[macro_use]
extern crate clap;

use async_minecraft_ping::ConnectionConfig;
use clap::{App, Arg};
use colored::*;

#[tokio::main]
async fn main() {
    // Handle CLI args
    let matches = App::new("Reverse Beacon Network Client")
        .version("0.1.2")
        .author("Evan Pratten <ewpratten@gmail.com>")
        .arg(
            Arg::with_name("server")
                .takes_value(true)
                .help("Minecraft server address")
                .required(true),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .takes_value(true)
                .help("Server port (Default: 25565)")
                .default_value("25565")
                .required(false),
        )
        .get_matches();

    // Get args
    let server = matches.value_of("server").unwrap();
    let port = value_t!(matches.value_of("port"), u16).unwrap_or(25565);

    // Define a connection to the server
    let connection_config = ConnectionConfig::build(server.to_string()).with_port(port);

    // Connect to the server
    let mut connection = connection_config.connect().await.unwrap();

    // Get status
    let status = connection.status().await.unwrap();

    // Print status
    println!("Minecraft {}", status.version.name.italic().bright_blue());
    if status.description.text.chars().count() > 0 {
        println!("{}", status.description.text.italic());
    }
    println!(
        "{} / {} players online",
        status.players.online, status.players.max
    );

    // Print player listing
    if status.players.sample.is_some() {
        for player in status.players.sample.unwrap() {
            println!(" - {}", player.name.bright_white().bold());
        }
    }
}
