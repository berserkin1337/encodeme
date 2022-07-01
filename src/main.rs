mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
use anyhow::{Ok, Result};
use clap::{App, AppSettings, Arg};
// TODO: Switch to Derive API for parsing args
fn main() -> Result<()> {
    let mut matches = App::new(env!("CARGO_CRATE_NAME"))
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            App::new("encode")
                .about("Encodes a message into a PNG image")
                // Automatically deduce the arguements based on their index.
                .arg(
                    Arg::new("path")
                        .takes_value(true)
                        .required(true)
                        .short('p')
                        .value_name("path")
                        .help("specify the path of the png image"),
                )
                .arg(
                    Arg::new("chunk_type")
                        .takes_value(true)
                        .required(true)
                        .short('t')
                        .value_name("chunk_type")
                        .help("specify the chunk type of the message"),
                )
                .arg(
                    Arg::new("message")
                        .takes_value(true)
                        .required(true)
                        .short('m')
                        .value_name("message")
                        .help("specify the message to encode"),
                )
                .arg(
                    Arg::new("output")
                        .takes_value(true)
                        .required(false)
                        .short('o')
                        .value_name("output")
                        .help("specify the path of the output png image"),
                ),
        )
        .subcommand(
            App::new("decode")
                .about("Decodes a PNG image into a message")
                .arg(
                    Arg::new("path")
                        .takes_value(true)
                        .required(true)
                        .short('p')
                        .value_name("chunk_type")
                        .help("specify the path of the png image"),
                )
                .arg(
                    Arg::new("chunk_type")
                        .takes_value(true)
                        .required(true)
                        .short('t')
                        .value_name("type")
                        .help("specify the chunk type of the message"),
                ),
        )
        .subcommand(
            App::new("remove")
                .about("Removes the secret message from the PNG image")
                .arg(
                    Arg::new("path")
                        .takes_value(true)
                        .required(true)
                        .short('p')
                        .value_name("path")
                        .help("specify the path of the png image"),
                )
                .arg(
                    Arg::new("chunk_type")
                        .takes_value(true)
                        .required(true)
                        .short('t')
                        .value_name("type")
                        .help("specify the chunk type of the message"),
                ),
        );
        // .subcommand(
        //     App::new("print").arg(
        //         Arg::new("path")
        //             .takes_value(true)
        //             .required(true)
        //             .short('p')
        //             .value_name("path")
        //             .help("specify the path of the png image"),
        //     ),
        // );
    let get_matches = matches.get_matches_mut();

    match get_matches.subcommand() {
        Some(("encode", sub_matches)) => {
            commands::encode(sub_matches)?;
        }
        Some(("decode", sub_matches)) => {
            commands::decode(sub_matches)?;
        }
        Some(("remove", sub_matches)) => {
            commands::remove(sub_matches)?;
        }
        // Some(("print", sub_matches)) => {
        //     commands::print(sub_matches)?;
        // }
        _ => {
            matches.write_help(&mut std::io::stdout()).expect(
                "
            failed to write to stdout",
            );
        }
    }

    Ok(())
}
