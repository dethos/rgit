use sha1::{Digest, Sha1};
use std::fs;
use std::path::Path;
use std::str;
use walkdir::WalkDir;

static RGIT_DIR: &'static str = ".rgit";

pub fn init() -> std::io::Result<()> {
    fs::create_dir(RGIT_DIR)?;
    fs::create_dir(format!("{}/{}", RGIT_DIR, "objects"))?;
    Ok(())
}

pub fn hash_object(content: &Vec<u8>, _type: String) -> String {
    let mut raw = format!("{}\u{0}", _type).into_bytes();
    let mut data = content.clone();
    raw.append(&mut data);

    let mut hasher = Sha1::new();
    hasher.update(&raw);
    let digest = &hasher.finalize();
    let s = format!("{:x}", digest);

    fs::write(format!("{}/{}/{}", RGIT_DIR, "objects", s), raw.as_slice())
        .expect("Failed to write object");

    return s;
}

pub fn get_object(hash: String, expected: String) -> String {
    let mut content = fs::read_to_string(format!("{}/{}/{}", RGIT_DIR, "objects", hash))
        .expect("Could not find a matching object");

    let index = content.find(char::from(0)).expect("object type missing");
    let data = content.split_off(index + 1);

    if expected != "".to_owned() {
        // Compare the type
        content.pop();
        assert_eq!(expected, content);
    }

    return data;
}

pub fn update_ref(reference: String, oid: String) {
    let path = format!("{}/{}", RGIT_DIR, reference);
    let mut parents = Path::new(&path).ancestors();
    parents.next();

    let parent = parents.next().unwrap().to_str().unwrap();
    fs::create_dir_all(parent).expect("Cannot create required dirs");
    fs::write(path, oid).expect("Failed to updated HEAD");
}

pub fn get_ref(reference: String) -> Result<String, Box<dyn std::error::Error + 'static>> {
    let ref_path = format!("{}/{}", RGIT_DIR, reference);
    let value = fs::read_to_string(ref_path)?;

    if !value.is_empty() && value.starts_with("ref:") {
        let new_ref: Vec<&str> = value.splitn(2, ":").collect();
        return get_ref(new_ref[1].to_owned());
    }
    return Ok(value);
}

pub fn iter_refs() -> Vec<(String, String)> {
    let mut refs: Vec<(String, String)> = vec![];
    refs.push(("HEAD".to_owned(), get_ref("HEAD".to_owned()).unwrap()));

    for entry in WalkDir::new(format!("{}/refs/", RGIT_DIR)) {
        let item = entry.unwrap();
        let metadata = item.metadata().unwrap();

        if metadata.is_file() {
            let relative_path = item.path().strip_prefix(RGIT_DIR).unwrap();
            refs.push((
                relative_path.to_str().unwrap().to_owned(),
                get_ref(relative_path.to_str().unwrap().to_owned()).unwrap(),
            ));
        }
    }

    return refs;
}
