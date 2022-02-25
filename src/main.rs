use async_minecraft_ping::{ConnectionConfig, ServerDescription};
use clap::{Arg, Command};
use colored::*;
use mc_legacy_formatting::SpanExt;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Handle CLI args
    let matches = Command::new("Minecraft Query")
        .version("1.0.1")
        .author("Evan Pratten <ewpratten@gmail.com>")
        .arg(
            Arg::new("server")
                .takes_value(true)
                .help("Minecraft server address")
                .required(true),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .takes_value(true)
                .help("Server port")
                .default_value("25565")
                .required(false),
        )
        .get_matches();

    // Get args
    let server = matches.value_of("server").unwrap();
    let port: u16 = matches.value_of_t("port").unwrap_or(25565);

    // Define a connection to the server
    let connection_config = ConnectionConfig::build(server.to_string()).with_port(port);

    // Connect to the server
    let connection = connection_config.connect().await.unwrap();

    // Get status
    let status = connection.status().await.unwrap().status;

    // Print status
    println!("Minecraft {}", status.version.name.italic().bright_blue());
    let description = match status.description {
        ServerDescription::Plain(str) => str,
        ServerDescription::Object { text } => text,
    };

    if !description.is_empty() {
        for span in description.span_iter() {
            print!("{}", span);
        }
        println!();
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
