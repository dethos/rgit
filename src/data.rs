use sha1::{Digest, Sha1};
use std::fs;
use std::str;

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

pub fn set_head(oid: String) {
    fs::write(format!("{}/HEAD", RGIT_DIR), oid).expect("Failed to updated HEAD");
}

pub fn get_head() -> Result<String, Box<dyn std::error::Error + 'static>> {
    let oid = fs::read_to_string(format!("{}/HEAD", RGIT_DIR))?;
    return Ok(oid);
}
