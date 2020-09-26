use clap::{App, SubCommand};

fn main() {
    let matches = App::new("rgit vcs")
        .version("0.1.0")
        .author("Gonçalo Valério <gon@ovalerio.net>")
        .about("A watered-down git clone")
        .subcommand(SubCommand::with_name("init").about("creates new repository"))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("init") {
        init();
    }
}

fn init() {
    println!("Hello, world!");
}
