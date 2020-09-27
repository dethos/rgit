use std::fs;

static RGIT_DIR: &'static str = ".rgit";

pub fn init() -> std::io::Result<()> {
    fs::create_dir(RGIT_DIR)?;
    Ok(())
}
