use std::collections::HashMap;
use std::fs;
use std::process::{Command, Stdio};
use tempfile::NamedTempFile;

#[path = "data.rs"]
mod data;

fn compare_trees(trees: Vec<HashMap<String, String>>) -> HashMap<String, Vec<String>> {
    let len_trees = trees.len();
    let mut entries: HashMap<String, Vec<String>> = HashMap::new();

    for (i, tree) in trees.iter().enumerate() {
        for (path, oid) in tree {
            if entries.contains_key(path) {
                entries.get_mut(path).unwrap()[i] = oid.clone();
            } else {
                entries.insert(path.clone(), vec!["".to_owned(); len_trees]);
                entries.get_mut(path).unwrap()[i] = oid.clone();
            }
        }
    }

    return entries;
}

fn diff_blobs(o_from: String, o_to: String, path: String) -> String {
    let f_from = NamedTempFile::new().unwrap();
    let f_to = NamedTempFile::new().unwrap();

    if o_from != "" {
        let content = data::get_object(o_from, "blob".to_owned());
        fs::write(f_from.path(), content).unwrap();
    }

    if o_to != "" {
        let content = data::get_object(o_to, "blob".to_owned());
        fs::write(f_to.path(), content).unwrap();
    }

    let output = Command::new("diff")
        .arg("--unified")
        .arg("--show-c-function")
        .arg("--label")
        .arg(format!("a/{}", path))
        .arg(f_from.path())
        .arg("--label")
        .arg(format!("a/{}", path))
        .arg(f_to.path())
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to launch diff");

    return String::from_utf8_lossy(&output.stdout).to_string();
}

pub fn diff_trees(t_from: HashMap<String, String>, t_to: HashMap<String, String>) -> String {
    let mut output = "".to_owned();
    let trees = vec![t_from, t_to];
    for (path, oids) in compare_trees(trees).iter() {
        let o_from = oids[0].clone();
        let o_to = oids[1].clone();
        if o_from != o_to {
            output.push_str(diff_blobs(o_from, o_to, path.clone()).as_str());
        }
    }

    return output;
}
