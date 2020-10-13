use std::fs;
#[path = "data.rs"]
mod data;

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

fn is_ignored(path: &String) -> bool {
    if path.contains(".rgit") {
        true
    } else {
        false
    }
}
