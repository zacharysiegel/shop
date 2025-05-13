use base64::Engine;
use clap::error::ErrorKind;
use clap::{Arg, ArgAction, ArgMatches, Command};
use crypt::cryptography::{decrypt, encrypt, generate_key};
use crypt::secrets::{list_secret_names, SecretBase64, BASE64};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut command: Command = Command::new("shop-crypt")
        .version("1.0")
        .about("Manage encrypted secrets for the shop application. Uses the ChaCha20Poly1305 algorithm.")
        .author("Zachary Siegel")
        .flatten_help(true)
        .subcommand(Command::new("encrypt")
            .about("Encrypt a secret with a key")
            .arg(Arg::new("key")
                .help("The base64-encoded secret key")
                .short('k')
                .long("key")
                .required(true))
            .arg(Arg::new("plaintext")
                .help("The plaintext to encrypt")
                .required(true)))
        .subcommand(Command::new("decrypt")
            .about("Decrypt a secret by name")
            .arg(Arg::new("key")
                .help("The base64-encoded secret key")
                .short('k')
                .long("key")
                .required(true))
            .arg(Arg::new("name")
                .help("The name of the encrypted secret")
                .required(true)))
        .subcommand(Command::new("key")
            .about("Generate a new encryption key"))
        .arg(Arg::new("list")
            .short('l')
            .long("list")
            .action(ArgAction::SetTrue)
            .exclusive(true)
            .help("List all available secrets"))
        ;
    let matches: ArgMatches = command.clone().get_matches();

    if matches.get_flag("list") {
        let text = list_secret_names()
            .join("\n");
        println!("{}", text);
        return Ok(());
    }

    if let Some(sub_matches) = matches.subcommand_matches("encrypt") {
        let plaintext: &String = sub_matches.get_one("plaintext")
            .expect("plaintext is required");
        let key: Vec<u8> = match_key(sub_matches)?;

        let secret: SecretBase64 = encrypt(&key, plaintext.as_bytes())?;

        println!("{}", secret);
        return Ok(());
    }

    if let Some(sub_matches) = matches.subcommand_matches("decrypt") {
        let secret_name: &String = sub_matches.get_one("name")
            .expect("secret name is required");
        let key: Vec<u8> = match_key(sub_matches)?;

        let plaintext: Vec<u8> = decrypt(&key, secret_name)?;

        if let Ok(plaintext_utf8) = String::from_utf8(plaintext.clone()) {
            println!("UTF-8 encoding:\n\t{}", plaintext_utf8);
        }
        println!("Base64 encoding:\n\t{}", BASE64.encode(&plaintext));
        return Ok(());
    }

    if let Some(_) = matches.subcommand_matches("key") {
        let key: Vec<u8> = generate_key();
        println!("Base64 encoding:\n\t{}", BASE64.encode(key));
        return Ok(());
    }

    command.error(ErrorKind::DisplayHelp, "Invalid invocation").exit()
}

fn match_key(matches: &ArgMatches) -> Result<Vec<u8>, Box<dyn Error>> {
    let key: &String = matches.get_one("key")
        .expect("key is required");
    Ok(BASE64.decode(key.as_bytes())?)
}
