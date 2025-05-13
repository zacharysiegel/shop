use base64::Engine;
use clap::error::ErrorKind;
use clap::{Arg, ArgAction, ArgGroup, ArgMatches, Command};
use crypt::cryptography;
use crypt::cryptography::{decrypt, encrypt, generate_key};
use crypt::secret::{list_secret_names, SecretBase64, BASE64};
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
                .long("key"))
            .arg(Arg::new("generate_key")
                .help("Generate a new key during encryption rather than accepting an existing key")
                .short('g')
                .long("generate-key")
                .action(ArgAction::SetTrue))
            .group(ArgGroup::new("keys")
                .args(["key", "generate_key"])
                .required(true)
                .multiple(false))
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

        let provided_key: Option<&String> = sub_matches.get_one("key");
        let generate_key: bool = sub_matches.get_flag("generate_key");

        if provided_key.is_some() && generate_key {
            command.error(ErrorKind::DisplayHelp, "key and generate_key are mutually exclusive").exit();
        }

        let key: Vec<u8> = match provided_key {
            Some(key) => BASE64.decode(key)?,
            None => cryptography::generate_key(),
        };

        let secret: SecretBase64 = encrypt(&key, plaintext.as_bytes())?;

        if generate_key {
            println!("Generated key (Base64):\n\t{}", BASE64.encode(&key));
        }
        println!("{}", secret);
        return Ok(());
    }

    if let Some(sub_matches) = matches.subcommand_matches("decrypt") {
        let secret_name: &String = sub_matches.get_one("name")
            .expect("secret name is required");
        let key: Vec<u8> = match sub_matches.get_one::<String>("key") {
            Some(key) => BASE64.decode(key.as_bytes())?,
            None => panic!("key is required"),
        };

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

    command.error(ErrorKind::DisplayHelp, "Invalid invocation").exit();
}
