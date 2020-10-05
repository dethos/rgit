use std::fs;

pub fn write_tree(directory: String) {
    let entries = fs::read_dir(&directory).unwrap();

    for entry in entries {
        let item = entry.unwrap();
        let metadata = item.metadata().unwrap();
        let name = item.file_name();
        let full = format!("{}/{}", directory, name.to_str().unwrap());
        if metadata.is_file() {
            println!("{}", full);
        } else if metadata.is_dir() {
            write_tree(full);
        }
    }
}
