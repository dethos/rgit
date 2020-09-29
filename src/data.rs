use sha1::{Digest, Sha1};
use std::fs;
use std::str;

static RGIT_DIR: &'static str = ".rgit";

pub fn init() -> std::io::Result<()> {
    fs::create_dir(RGIT_DIR)?;
    fs::create_dir(format!("{}/{}", RGIT_DIR, "objects"))?;
    Ok(())
}

pub fn hash_object(content: &Vec<u8>) -> String {
    let mut hasher = Sha1::new();
    hasher.update(content);
    let digest = &hasher.finalize();
    let s = format!("{:x}", digest);

    fs::write(
        format!("{}/{}/{}", RGIT_DIR, "objects", s),
        content.as_slice(),
    )
    .expect("Failed to write object");

    return s;
}

pub fn get_object(hash: String) -> String {
    return fs::read_to_string(format!("{}/{}/{}", RGIT_DIR, "objects", hash))
        .expect("Could not find a matching object");
}
