use std::fs;
#[path = "data.rs"]
mod data;

pub fn write_tree(directory: String) {
    let entries = fs::read_dir(&directory).unwrap();

    for entry in entries {
        let item = entry.unwrap();
        let metadata = item.metadata().unwrap();
        let name = item.file_name();
        let full = format!("{}/{}", directory, name.to_str().unwrap());

        if is_ignored(&full) {
            continue;
        }

        if metadata.is_file() {
            let hash = data::hash_object(&fs::read(&full).unwrap(), "blob".to_owned());
            println!("{} {}", hash, full);
        } else if metadata.is_dir() {
            write_tree(full);
        }
    }
}

fn is_ignored(path: &String) -> bool {
    if path.contains(".rgit") {
        true
    } else {
        false
    }
}
