use std::collections::HashMap;

#[path = "data.rs"]
mod data;

#[path = "base.rs"]
mod base;

static REMOTE_REFS_BASE: &'static str = "refs/heads/";
static LOCAL_REFS_BASE: &'static str = "refs/remote/";

pub fn fetch(path: String) {
    // Get refs from server
    let refs = get_remote_refs(path.clone(), REMOTE_REFS_BASE);

    let commit_oids: Vec<&String> = refs.values().collect();
    base::copy_objects_in_commits_and_parents(commit_oids, path.clone(), false);

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

pub fn push(remote_path: String, reference: String) {
    let refs = get_remote_refs(remote_path.clone(), REMOTE_REFS_BASE);
    let empty = "".to_owned();
    let remote_ref = refs.get(&reference).unwrap_or(&empty);
    let local_ref = data::get_ref(reference.clone(), true).value;
    assert!(local_ref != "".to_string());

    // Don't allow force push
    assert!(
        *remote_ref == "".to_owned() || base::is_ancestor_of(local_ref.clone(), remote_ref.clone())
    );

    let commit_oids = vec![&local_ref];
    base::copy_objects_in_commits_and_parents(commit_oids, remote_path.clone(), true);

    data::set_rgit_dir(remote_path.as_str());
    data::update_ref(
        reference,
        data::RefValue {
            symbolic: false,
            value: local_ref,
        },
        true,
    );
    data::reset_rgit_dir();
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
