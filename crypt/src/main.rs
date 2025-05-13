use base64::Engine;
use clap::{Arg, ArgAction, ArgMatches, Command};
use crypt::secrets::{decrypt, encrypt, list_secrets, Secret, BASE64};
use std::error::Error;

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
        let secret_name: &String = sub_matches.get_one("name")
            .expect("secret name is required");
        let plaintext: Vec<u8> = decrypt(secret_name)?;

        if let Ok(plaintext_utf8) = String::from_utf8(plaintext.clone()) {
            println!("UTF-8 encoding:\n\t{}", plaintext_utf8);
        }
        println!("Base64 encoding:\n\t{}", BASE64.encode(&plaintext));
        return Ok(());
    }

    Err("Failed to detect invalid subcommand".into())
}
