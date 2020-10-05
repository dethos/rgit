use clap::{App, Arg, ArgMatches, SubCommand};
use std::fs;
mod base;
mod data;

fn main() {
    let matches = App::new("rgit vcs")
        .version("0.1.0")
        .author("Gonçalo Valério <gon@ovalerio.net>")
        .about("A watered-down git clone")
        .subcommand(SubCommand::with_name("init").about("creates new repository"))
        .subcommand(
            SubCommand::with_name("hash-object")
                .about("created an hash for an object")
                .arg(Arg::with_name("file").index(1).required(true)),
        )
        .subcommand(
            SubCommand::with_name("cat-file")
                .about("outputs the original object from the provided hash")
                .arg(Arg::with_name("hash").index(1).required(true)),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("init") => init(),
        Some("hash-object") => hash_object(matches),
        Some("cat-file") => cat_file(matches),
        _ => println!("unknown sub command"),
    }
}

fn init() {
    match data::init() {
        Ok(()) => println!("Repository created"),
        _ => println!("Failed. Perhaps the repository already exists."),
    }
}

fn hash_object(matches: ArgMatches) {
    if let Some(cmd_matches) = matches.subcommand_matches("hash-object") {
        let content = fs::read(cmd_matches.value_of("file").unwrap())
            .expect("Something went wrong reading the provided file");
        let hash = data::hash_object(&content, "blob".to_owned());
        println!("{}", hash);
    }
}

fn cat_file(matches: ArgMatches) {
    if let Some(cmd_matches) = matches.subcommand_matches("cat-file") {
        let file_contents = data::get_object(
            cmd_matches.value_of("hash").unwrap().to_owned(),
            "".to_owned(),
        );
        println!("{}", file_contents)
    }
}
