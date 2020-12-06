use lazy_static::lazy_static;
use sha1::{Digest, Sha1};
use std::fs;
use std::path::Path;
use std::str;
use std::sync::Mutex;
use walkdir::WalkDir;

static BASE_RGIT_DIR: &'static str = ".rgit";

lazy_static! {
    static ref RGIT_DIR: Mutex<String> = Mutex::new(BASE_RGIT_DIR.to_owned());
    static ref OLD_DIR: Mutex<String> = Mutex::new("".to_owned());
}

pub struct RefValue {
    pub value: String,
    pub symbolic: bool,
}

// The below two methods are not the same thing as a "context manager"
// I might need to replace it later with a better alternative.
pub fn set_rgit_dir(path: &str) {
    let mut dir = RGIT_DIR.lock().unwrap();
    let mut old_dir = OLD_DIR.lock().unwrap();

    *old_dir = dir.to_string();
    *dir = format!("{}/.rgit", path);
}

pub fn reset_rgit_dir() {
    let mut dir = RGIT_DIR.lock().unwrap();
    let mut old_dir = OLD_DIR.lock().unwrap();
    if old_dir.to_string() == "" {
        *dir = BASE_RGIT_DIR.to_string().clone();
    } else {
        *dir = old_dir.to_string();
        *old_dir = "".to_string();
    }
}

pub fn init() -> std::io::Result<()> {
    let dir = RGIT_DIR.lock().unwrap().to_owned();
    fs::create_dir(dir.clone())?;
    fs::create_dir(format!("{}/{}", dir, "objects"))?;
    Ok(())
}

pub fn hash_object(content: &Vec<u8>, _type: String) -> String {
    let dir = RGIT_DIR.lock().unwrap().to_owned();
    let mut raw = format!("{}\u{0}", _type).into_bytes();
    let mut data = content.clone();
    raw.append(&mut data);

    let mut hasher = Sha1::new();
    hasher.update(&raw);
    let digest = &hasher.finalize();
    let s = format!("{:x}", digest);

    fs::write(format!("{}/{}/{}", dir, "objects", s), raw.as_slice())
        .expect("Failed to write object");

    return s;
}

pub fn get_object(hash: String, expected: String) -> String {
    let dir = RGIT_DIR.lock().unwrap().to_owned();
    let mut content = fs::read_to_string(format!("{}/{}/{}", dir, "objects", hash))
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

pub fn update_ref(mut reference: String, value: RefValue, deref: bool) {
    let dir = RGIT_DIR.lock().unwrap().to_owned();
    reference = get_ref_internal(reference, deref).0;
    let content: String;

    assert!(value.value != "");
    if value.symbolic {
        content = format!("ref: {}", value.value);
    } else {
        content = value.value;
    }

    let path = format!("{}/{}", dir, reference);
    let mut parents = Path::new(&path).ancestors();
    parents.next();

    let parent = parents.next().unwrap().to_str().unwrap();
    fs::create_dir_all(parent).expect("Cannot create required dirs");
    fs::write(path, content).expect("Failed to updated HEAD");
}

pub fn get_ref(reference: String, deref: bool) -> RefValue {
    return get_ref_internal(reference, deref).1;
}

pub fn delete_ref(reference: String, deref: bool) {
    let dir = RGIT_DIR.lock().unwrap().to_owned();
    let ref_to_del = get_ref_internal(reference, deref).0;
    fs::remove_file(format!("{}/{}", dir, ref_to_del)).unwrap();
}

pub fn iter_refs(prefix: &str, deref: bool) -> Vec<(String, RefValue)> {
    let dir = RGIT_DIR.lock().unwrap().to_owned();
    let mut refs: Vec<(String, RefValue)> = vec![];

    refs.push(("HEAD".to_owned(), get_ref("HEAD".to_owned(), deref)));
    refs.push((
        "MERGE_HEAD".to_owned(),
        get_ref("MERGE_HEAD".to_owned(), deref),
    ));

    for entry in WalkDir::new(format!("{}/refs/", dir.clone())) {
        let item = entry.unwrap();
        let metadata = item.metadata().unwrap();

        if metadata.is_file() {
            let relative_path = item.path().strip_prefix(dir.clone()).unwrap();
            refs.push((
                relative_path.to_str().unwrap().to_owned(),
                get_ref(relative_path.to_str().unwrap().to_owned(), deref),
            ));
        }
    }

    let mut filtered_refs = vec![];
    for reference in refs {
        if reference.0.starts_with(prefix) && reference.1.value != "".to_owned() {
            filtered_refs.push(reference);
        }
    }
    return filtered_refs;
}

pub fn get_ref_internal(reference: String, deref: bool) -> (String, RefValue) {
    let dir = RGIT_DIR.lock().unwrap().to_owned();
    let ref_path = format!("{}/{}", dir, reference);
    let mut value = fs::read_to_string(ref_path).unwrap_or("".to_owned());
    let symbolic = !value.is_empty() && value.starts_with("ref:");

    if symbolic {
        let new_ref: Vec<&str> = value.splitn(2, ": ").collect();
        value = new_ref[1].to_owned();
        if deref {
            return get_ref_internal(value, deref);
        }
    }

    return (reference, RefValue { value, symbolic });
}

pub fn fetch_object_if_missing(oid: String, remote_git_dir: String) {
    if object_exists(oid.clone()) {
        return;
    }

    let dir = RGIT_DIR.lock().unwrap().to_owned();
    let rgit_remote = remote_git_dir + "/.rgit";
    fs::copy(
        format!("{}/objects/{}", rgit_remote, oid.clone()),
        format!("{}/objects/{}", dir, oid),
    )
    .expect(format!("Failed to fetch {}", oid).as_str());
}

pub fn push_object(oid: String, remote_git_dir: String) {
    let rgit_remote = remote_git_dir + "/.rgit";
    let remote_object = format!("{}/objects/{}", rgit_remote, oid.clone());

    if Path::new(&remote_object).exists() {
        // Only push object if it doesn't exist.
        // Different implementation from the tutorial, the end result should be
        // the same.
        return;
    }

    let dir = RGIT_DIR.lock().unwrap().to_owned();
    fs::copy(format!("{}/objects/{}", dir, oid), remote_object)
        .expect(format!("Failed to push {}", oid).as_str());
}

fn object_exists(oid: String) -> bool {
    let dir = RGIT_DIR.lock().unwrap().to_owned();
    let path = format!("{}/objects/{}", dir.clone(), oid.clone());
    return Path::new(path.as_str()).exists();
}
