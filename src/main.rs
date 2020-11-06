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
        .subcommand(
            SubCommand::with_name("write-tree")
                .about("write the current working directory to the database"),
        )
        .subcommand(
            SubCommand::with_name("read-tree")
                .about("writes a given tree to the working directory")
                .arg(Arg::with_name("oid").index(1).required(true)),
        )
        .subcommand(
            SubCommand::with_name("commit")
                .about("writes a named snapshot of the current tree")
                .arg(
                    Arg::with_name("message")
                        .short("m")
                        .value_name("MESSAGE")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("log")
                .about("List all commits")
                .arg(Arg::with_name("oid").index(1).required(false)),
        )
        .subcommand(
            SubCommand::with_name("checkout")
                .about("Move the current content and HEAD to given commit")
                .arg(Arg::with_name("oid").index(1).required(true)),
        )
        .subcommand(
            SubCommand::with_name("tag")
                .about("Create a tag for a given commit")
                .arg(Arg::with_name("name").index(1).required(true))
                .arg(Arg::with_name("oid").index(2).required(false)),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("init") => init(),
        Some("hash-object") => hash_object(matches),
        Some("cat-file") => cat_file(matches),
        Some("write-tree") => write_tree(),
        Some("read-tree") => read_tree(matches),
        Some("commit") => commit(matches),
        Some("log") => log_commits(matches),
        Some("checkout") => checkout(matches),
        Some("tag") => tag(matches),
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
        let hash = base::get_oid(cmd_matches.value_of("hash").unwrap().to_owned());
        let file_contents = data::get_object(hash, "".to_owned());
        println!("{}", file_contents)
    }
}

fn write_tree() {
    println!("{}", base::write_tree(".".to_owned()));
}

fn read_tree(matches: ArgMatches) {
    if let Some(cmd_matches) = matches.subcommand_matches("read-tree") {
        let oid = base::get_oid(cmd_matches.value_of("oid").unwrap().to_owned());
        base::read_tree(oid);
    }
}

fn commit(matches: ArgMatches) {
    if let Some(cmd_matches) = matches.subcommand_matches("commit") {
        let message = cmd_matches.value_of("message").unwrap_or("");
        println!("{}", base::commit(message));
    }
}

fn log_commits(matches: ArgMatches) {
    if let Some(cmd_matches) = matches.subcommand_matches("log") {
        let provided_ref = cmd_matches.value_of("oid").unwrap_or("").to_owned();
        let mut oid;
        if provided_ref == "" {
            oid = base::get_oid("HEAD".to_owned());
        } else {
            oid = base::get_oid(provided_ref.to_owned());
        }

        loop {
            let commit = base::get_commit(oid.clone());

            println!("commit {}", oid);
            println!("{}", commit.message);
            println!("");

            if commit.parent == "" {
                break;
            }

            oid = commit.parent;
        }
    }
}

fn checkout(matches: ArgMatches) {
    if let Some(cmd_matches) = matches.subcommand_matches("checkout") {
        let oid = base::get_oid(cmd_matches.value_of("oid").unwrap().to_owned());
        base::checkout(oid);
    }
}

fn tag(matches: ArgMatches) {
    if let Some(cmd_matches) = matches.subcommand_matches("tag") {
        let name = cmd_matches.value_of("name").unwrap().to_owned();
        let mut oid = cmd_matches.value_of("oid").unwrap_or("").to_owned();
        if oid == "" {
            oid = base::get_oid("HEAD".to_owned());
        } else {
            oid = base::get_oid(name.clone());
        }
        base::create_tag(name, oid);
    }
}
