use clap::{App, Arg, ArgMatches, SubCommand};
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};
mod base;
mod data;
mod diff;

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
                .arg(Arg::with_name("oid").index(1).default_value("@")),
        )
        .subcommand(
            SubCommand::with_name("checkout")
                .about("Move the current content and HEAD to given commit")
                .arg(Arg::with_name("commit").index(1).required(true)),
        )
        .subcommand(
            SubCommand::with_name("tag")
                .about("Create a tag for a given commit")
                .arg(Arg::with_name("name").index(1).required(true))
                .arg(Arg::with_name("oid").index(2).default_value("@")),
        )
        .subcommand(SubCommand::with_name("k").about("visualize refs and commits"))
        .subcommand(
            SubCommand::with_name("branch")
                .about("Create a new branch")
                .arg(Arg::with_name("name").index(1).required(false))
                .arg(Arg::with_name("start_point").index(2).default_value("@")),
        )
        .subcommand(SubCommand::with_name("status").about("check current branch"))
        .subcommand(
            SubCommand::with_name("reset")
                .about("Move the current content and HEAD to given commit with dereferencing")
                .arg(Arg::with_name("commit").index(1).required(true)),
        )
        .subcommand(
            SubCommand::with_name("show")
                .about("Show diff from a commit")
                .arg(Arg::with_name("oid").index(1).default_value("@")),
        )
        .subcommand(
            SubCommand::with_name("diff")
                .about("Compare the working tree with the given commit")
                .arg(Arg::with_name("commit").index(1).default_value("@")),
        )
        .subcommand(
            SubCommand::with_name("merge")
                .about("Merge changes of a different commit/branch")
                .arg(Arg::with_name("commit").index(1).required(true)),
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
        Some("k") => k(),
        Some("branch") => branch(matches),
        Some("status") => status(),
        Some("reset") => reset(matches),
        Some("show") => show(matches),
        Some("diff") => difference(matches),
        Some("merge") => merge(matches),
        _ => println!("unknown sub command"),
    }
}

fn init() {
    match base::init() {
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
        let provided_ref = cmd_matches.value_of("oid").unwrap().to_owned();

        let mut refs: HashMap<String, Vec<String>> = HashMap::new();
        for entry in data::iter_refs("", true) {
            if refs.contains_key(&entry.1.value) {
                refs.get_mut(&entry.1.value).unwrap().push(entry.0);
            } else {
                refs.insert(entry.1.value, vec![entry.0]);
            }
        }

        let initial_oid = base::get_oid(provided_ref.to_owned());
        let mut oids = VecDeque::new();
        oids.push_back(initial_oid);

        for oid in base::iter_commits_and_parents(oids) {
            let commit = base::get_commit(oid.clone());

            print_commit(oid, &commit, refs.clone());

            if commit.parents[0] == "" {
                break;
            }
        }
    }
}

fn checkout(matches: ArgMatches) {
    if let Some(cmd_matches) = matches.subcommand_matches("checkout") {
        let name = cmd_matches.value_of("commit").unwrap().to_owned();
        base::checkout(name);
    }
}

fn tag(matches: ArgMatches) {
    if let Some(cmd_matches) = matches.subcommand_matches("tag") {
        let name = cmd_matches.value_of("name").unwrap().to_owned();
        let provided_ref = cmd_matches.value_of("oid").unwrap().to_owned();
        let oid = base::get_oid(provided_ref.clone());
        base::create_tag(name, oid);
    }
}

fn k() {
    let mut dot = "digraph commits {\n".to_owned();
    let mut oids = VecDeque::new();
    for refinfo in data::iter_refs("", false) {
        dot.push_str(&format!("\"{}\" [shape=note]\n", refinfo.0));
        dot.push_str(&format!("\"{}\" -> \"{}\"", refinfo.0, refinfo.1.value));
        if !refinfo.1.symbolic {
            oids.push_back(refinfo.1.value);
        }
    }

    for oid in base::iter_commits_and_parents(oids) {
        let commit = base::get_commit(oid.clone());
        dot.push_str(&format!(
            "\"{}\" [shape=box style=filled label=\"{}\"]\n",
            oid,
            &oid[0..10]
        ));
        if commit.parents[0] != "" {
            println!("Parent: {}", commit.parents[0]);
            dot.push_str(&format!("\"{}\" -> \"{}\"\n", oid, commit.parents[0]));
        }
    }
    dot.push_str("}");
    println!("{}", dot);

    let mut child = Command::new("dot")
        .arg("-Tgtk")
        .arg("/dev/stdin")
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to draw graph");

    {
        let stdin = child.stdin.as_mut().expect("Cannot get dot stdin");
        stdin
            .write_all(dot.into_bytes().as_mut_slice())
            .expect("failed to write graph data");
    }
    let _ = child.wait();
}

fn branch(matches: ArgMatches) {
    if let Some(cmd_matches) = matches.subcommand_matches("branch") {
        let name = cmd_matches.value_of("name").unwrap_or("").to_owned();
        let provided_ref = cmd_matches.value_of("start_point").unwrap().to_owned();
        if name == "" {
            let current = base::get_branch_name();
            for branch in base::iter_branch_names() {
                let prefix = if branch == current { "*" } else { " " };
                println!("{} {}", prefix, branch);
            }
        } else {
            let oid = base::get_oid(provided_ref.clone());
            base::create_branch(name.clone(), oid.clone());
            println!("Branch {} created_at {}", name, oid);
        }
    }
}

fn status() {
    let branch = base::get_branch_name();
    let head = base::get_oid("@".to_owned());
    if branch != "".to_owned() {
        println!("On branch {}", branch);
    } else {
        println!("HEAD detached at {}", &head[1..10])
    }

    println!("Changes to be committed:\n");
    let head_commit = base::get_commit(head);
    for (path, action) in diff::changed_files(
        base::get_tree(head_commit.tree, "".to_owned()),
        base::get_working_tree(),
    ) {
        println!("{:>12}: {}", action, path);
    }
}

fn reset(matches: ArgMatches) {
    if let Some(cmd_matches) = matches.subcommand_matches("reset") {
        let oid = base::get_oid(cmd_matches.value_of("commit").unwrap().to_owned());
        base::reset(oid);
    }
}

fn show(matches: ArgMatches) {
    if let Some(cmd_matches) = matches.subcommand_matches("show") {
        let oid = base::get_oid(cmd_matches.value_of("oid").unwrap().to_owned());
        let commit = base::get_commit(oid.clone());
        let refs: HashMap<String, Vec<String>> = HashMap::new();
        let parent_tree = if commit.parents[0] != "".to_owned() {
            base::get_commit(commit.parents[0].clone()).tree
        } else {
            "".to_owned()
        };

        print_commit(oid, &commit, refs);
        let result = diff::diff_trees(
            base::get_tree(parent_tree, "".to_owned()),
            base::get_tree(commit.tree, "".to_owned()),
        );
        println!("{}", result);
    }
}

fn difference(matches: ArgMatches) {
    if let Some(cmd_matches) = matches.subcommand_matches("diff") {
        let oid = base::get_oid(cmd_matches.value_of("commit").unwrap().to_owned());
        let commit = base::get_commit(oid);
        let result = diff::diff_trees(
            base::get_tree(commit.tree, "".to_owned()),
            base::get_working_tree(),
        );
        println!("{}", result);
    }
}

fn merge(matches: ArgMatches) {
    if let Some(cmd_matches) = matches.subcommand_matches("reset") {
        let oid = base::get_oid(cmd_matches.value_of("commit").unwrap().to_owned());
        base::merge(oid);
    }
}

fn print_commit(oid: String, commit: &base::Commit, mut refs: HashMap<String, Vec<String>>) {
    let ref_str = if refs.contains_key(&oid) {
        refs.get_mut(&oid).unwrap().join(", ")
    } else {
        "".to_owned()
    };

    println!("commit {} {}", oid, ref_str);
    println!("{}", commit.message);
    println!("");
}
