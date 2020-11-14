use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::io;
use std::path::Path;
#[path = "data.rs"]
mod data;

pub struct Commit {
    pub tree: String,
    pub parent: String,
    pub message: String,
}

pub fn write_tree(directory: String) -> String {
    let mut entries: Vec<(String, String, String)> = vec![];
    let mut name;
    let mut type_: String;
    let mut oid: String;

    let it = fs::read_dir(&directory).unwrap();
    for entry in it {
        let item = entry.unwrap();
        let metadata = item.metadata().unwrap();
        name = item.file_name();
        let full = format!("{}/{}", directory, name.to_str().unwrap());
        if is_ignored(&full) {
            continue;
        }

        if metadata.is_file() {
            type_ = "blob".to_owned();
            oid = data::hash_object(&fs::read(&full).unwrap(), type_.clone());
            println!("{} {}", oid, full);
        } else if metadata.is_dir() {
            type_ = "tree".to_owned();
            oid = write_tree(full);
        } else {
            panic!("What is this?");
        }
        entries.push((
            name.to_str().unwrap().to_owned().clone(),
            oid.clone(),
            type_.clone(),
        ));
    }

    entries.sort();

    let mut tree = String::new();
    for entry in entries {
        tree.push_str(&format!("{} {} {}\n", entry.2, entry.1, entry.0));
    }

    return data::hash_object(&tree.into_bytes(), "tree".to_owned());
}

pub fn read_tree(oid: String) {
    empty_current_directory(".").unwrap();
    for (path, object_id) in get_tree(oid, "./".to_owned()).iter() {
        let mut dirs = Path::new(path).ancestors();
        dirs.next();

        let dir = dirs.next().unwrap().to_str().unwrap();

        fs::create_dir_all(dir).expect("Cannot create required dirs");
        fs::write(path, data::get_object(object_id.clone(), "".to_owned()))
            .expect("Cannot write required object");
    }
}

pub fn commit(message: &str) -> String {
    let mut commit = format!("tree {}\n", write_tree(".".to_owned()));

    let head = data::get_ref("HEAD".to_owned(), true);
    if head.value != "" {
        commit += format!("parent {}\n", head.value).as_str();
    }

    commit += "\n";
    commit += format!("{}\n", message).as_str();

    let oid = data::hash_object(&commit.into_bytes(), "commit".to_owned());
    data::update_ref(
        "HEAD".to_owned(),
        data::RefValue {
            value: oid.clone(),
            symbolic: false,
        },
        true,
    );
    return oid;
}

pub fn get_commit(oid: String) -> Commit {
    let commit = data::get_object(oid, "commit".to_owned());
    let tree: String;
    let mut parent: String = "".to_owned();
    let message: String;
    let mut message_start = 2;

    let lines: Vec<&str> = commit.lines().collect();
    let mut line_items: Vec<&str> = lines[0].splitn(2, " ").collect();
    tree = line_items[1].to_owned();

    line_items = lines[1].splitn(2, " ").collect();
    if line_items[0] == "parent" {
        parent = line_items[1].to_owned();
        message_start = 3;
    }

    message = lines[message_start..].join("\n");

    return Commit {
        tree,
        parent,
        message,
    };
}

pub fn iter_commits_and_parents(mut oids: VecDeque<String>) -> Vec<String> {
    let mut visited: HashSet<String> = HashSet::new();
    let mut oid_sequence = vec![];

    while !oids.is_empty() {
        let oid = oids.pop_front().unwrap();
        if oid == "" || visited.contains(&oid) {
            continue;
        }
        visited.insert(oid.clone());
        oid_sequence.push(oid.clone());

        let commit = get_commit(oid);
        // Deal with parent next
        oids.push_front(commit.parent);
    }

    return oid_sequence;
}

pub fn checkout(name: String) {
    let oid = get_oid(name.clone());
    let commit = get_commit(oid.clone());
    let head;
    read_tree(commit.tree);

    if is_branch(name.clone()) {
        head = data::RefValue {
            symbolic: true,
            value: format!("refs/heads/{}", name),
        };
    } else {
        head = data::RefValue {
            symbolic: false,
            value: oid,
        };
    }

    data::update_ref("HEAD".to_owned(), head, false);
}

pub fn create_tag(name: String, oid: String) {
    data::update_ref(
        format!("refs/tags/{}", name),
        data::RefValue {
            value: oid,
            symbolic: false,
        },
        true,
    );
}

pub fn get_oid(mut name: String) -> String {
    if name == "@".to_owned() {
        name = "HEAD".to_owned();
    }

    let refs_to_try: [String; 4] = [
        format!("{}", name),
        format!("refs/{}", name),
        format!("refs/tags/{}", name),
        format!("refs/heads/{}", name),
    ];

    for reference in refs_to_try.into_iter() {
        let found = data::get_ref(reference.clone(), false);
        if found.value != "" {
            return found.value;
        } else {
            continue;
        }
    }

    let mut is_hex = true;
    for c in name.chars() {
        if !c.is_ascii_hexdigit() {
            is_hex = false;
        }
    }

    if name.len() == 40 && is_hex {
        return name;
    }

    panic!(format!("Unknown name {}", name));
}

pub fn create_branch(name: String, oid: String) {
    data::update_ref(
        format!("refs/heads/{}", name),
        data::RefValue {
            value: oid,
            symbolic: false,
        },
        true,
    );
}

pub fn init() -> std::io::Result<()> {
    data::init()?;
    data::update_ref(
        "HEAD".to_owned(),
        data::RefValue {
            symbolic: true,
            value: "refs/heads/master".to_owned(),
        },
        true,
    );
    Ok(())
}

pub fn get_branch_name() -> String {
    let head = data::get_ref("HEAD".to_owned(), false);
    if !head.symbolic {
        return "".to_owned();
    }
    assert!(head.value.starts_with("refs/heads/"));
    let ref_items: Vec<&str> = head.value.splitn(3, "/").collect();
    return (*ref_items.last().unwrap()).to_owned();
}

fn is_ignored(path: &String) -> bool {
    if path.contains(".rgit") {
        true
    } else {
        false
    }
}

fn tree_entries(oid: String) -> Vec<(String, String, String)> {
    let mut entries: Vec<(String, String, String)> = vec![];
    let tree_data = data::get_object(oid, "tree".to_owned());
    for line in tree_data.split_terminator("\n") {
        let items: Vec<&str> = line.splitn(3, " ").collect();
        entries.push((
            items[0].to_owned(), // _type
            items[1].to_owned(), // oid
            items[2].to_owned(), // name
        ));
    }
    return entries;
}

fn get_tree(oid: String, base_path: String) -> HashMap<String, String> {
    let mut result = HashMap::new();
    for entry in tree_entries(oid) {
        // _type, oid, name
        assert!(entry.2.find("/").is_none());
        assert!(entry.2 != "..");
        assert!(entry.2 != ".");
        let path = base_path.clone() + entry.2.as_str();
        if entry.0 == "blob".to_owned() {
            result.insert(path.clone(), entry.1.clone());
        } else if entry.0 == "tree".to_owned() {
            result.extend(get_tree(entry.1, format!("{}/", path)));
        } else {
            panic!("Unknown tree entry: {}", entry.0);
        }
    }
    result
}

fn empty_current_directory(dir: &str) -> io::Result<()> {
    // Delete current directory, except the ignored directories and files
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if is_ignored(&path.to_str().unwrap().to_owned()) {
            continue;
        }

        if path.is_dir() {
            empty_current_directory(path.to_str().unwrap())?;
            match fs::remove_dir(&path) {
                Ok(()) => (),
                _ => println!("Unable to remove dir {}", path.clone().to_str().unwrap()),
            };
        } else {
            fs::remove_file(&path)?
        }
    }
    Ok(())
}

fn is_branch(name: String) -> bool {
    return data::get_ref(name, true).value != "";
}
