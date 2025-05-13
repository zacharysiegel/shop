use clap::{Arg, ArgAction, ArgMatches, Command};
use crypt::secrets::list_secrets;

fn main() {
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
        return;
    }
    //
    // let x = encrypt(b"");
    // let y = decrypt(&x.0, &x.1, x.2.unwrap().as_slice());
}
