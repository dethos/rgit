use std::collections::HashMap;

#[path = "data.rs"]
mod data;

pub fn fetch(path: String) {
    println!("Will fetch the following refs:");
    for (refname, _) in get_remote_refs(path, "refs/heads").iter() {
        println!("- {}", refname);
    }
}

fn get_remote_refs(path: String, prefix: &str) -> HashMap<String, String> {
    let mut refs = HashMap::new();
    data::set_rgit_dir(path.as_str());
    for (refname, reference) in data::iter_refs(prefix, true) {
        refs.insert(refname, reference.value);
    }
    data::reset_rgit_dir();
    return refs;
}
