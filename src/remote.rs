use std::collections::HashMap;

#[path = "data.rs"]
mod data;

static REMOTE_REFS_BASE: &'static str = "refs/heads/";
static LOCAL_REFS_BASE: &'static str = "refs/remote/";

pub fn fetch(path: String) {
    // Get refs from server
    let refs = get_remote_refs(path, REMOTE_REFS_BASE);

    // Update local refs to match server
    for (remote_name, value) in refs.iter() {
        let refname = remote_name.trim_start_matches(REMOTE_REFS_BASE);
        data::update_ref(
            format!("{}/{}", LOCAL_REFS_BASE, refname),
            data::RefValue {
                symbolic: false,
                value: value.clone(),
            },
            true,
        )
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
