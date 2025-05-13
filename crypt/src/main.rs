use std::error::Error;
use base64::{DecodeError, Engine};
use chacha20poly1305::Key;
use clap::{Arg, ArgAction, ArgMatches, Command};
use clap::error::ErrorKind;
use crypt::secrets;
use crypt::secrets::{encrypt, list_secrets, Secret, BASE64};

fn main() -> Result<(), Box<dyn Error>> {
    let command: Command = Command::new("shop-crypt")
        .version("1.0")
        .about("Manage encrypted secrets for the shop application")
        .author("Zachary Siegel")
        .next_line_help(true)
        .flatten_help(true)
        .subcommand(Command::new("encrypt")
            .next_line_help(true)
            .arg(Arg::new("plaintext")
                .help("The plaintext to encrypt")
                .required(true)))
        .subcommand(Command::new("decrypt")
            .next_line_help(true)
            .arg(Arg::new("name")
                .help("The name of the encrypted secret")
                .required(true)))
        .arg(Arg::new("list")
            .short('l')
            .long("list")
            .action(ArgAction::SetTrue)
            .exclusive(true)
            .help("List all available secrets"))
        ;
    let matches: ArgMatches = command.get_matches();


    if matches.get_flag("list") {
        let text = list_secrets()
            .join("\n");
        println!("{}", text);
        return Ok(());
    }

    if let Some(sub_matches) = matches.subcommand_matches("encrypt") {
        let plaintext: &String = sub_matches.get_one("plaintext")
            .expect("plaintext is required");

        let secret: Secret = encrypt(plaintext.as_bytes())?;
        println!("{}", secret);
        return Ok(());
    }

    if let Some(sub_matches) = matches.subcommand_matches("decrypt") {

    }

    unreachable!()
}
