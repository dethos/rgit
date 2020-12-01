#[path = "data.rs"]
mod data;

pub fn fetch(path: String) {
    println!("Will fetch the following refs:");
    data::set_rgit_dir(path.as_str());
    for (refname, _) in data::iter_refs("refs/heads", true) {
        println!(" - {}", refname);
    }
    data::reset_rgit_dir();
}
