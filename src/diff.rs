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

pub fn changed_files(
    t_from: HashMap<String, String>,
    t_to: HashMap<String, String>,
) -> Vec<(String, String)> {
    let mut result = vec![];
    let trees = vec![t_from, t_to];
    for (path, oids) in compare_trees(trees).iter() {
        let o_from = oids[0].clone();
        let o_to = oids[1].clone();
        if o_from != o_to {
            let action = if o_from == "" {
                "new file"
            } else if o_to == "" {
                "deleted"
            } else {
                "mofified"
            };
            result.push((path.clone(), action.to_owned()))
        }
    }
    return result;
}

pub fn merge_trees(
    t_base: HashMap<String, String>,
    t_head: HashMap<String, String>,
    t_other: HashMap<String, String>,
) -> HashMap<String, String> {
    let mut tree = HashMap::new();
    let trees = vec![t_base, t_head, t_other];
    for (path, oids) in compare_trees(trees).iter() {
        tree.insert(
            path.clone(),
            merge_blobs(oids[0].clone(), oids[1].clone(), oids[2].clone()),
        );
    }
    return tree;
}

fn merge_blobs(o_base: String, o_head: String, o_other: String) -> String {
    let f_base = NamedTempFile::new().unwrap();
    let f_head = NamedTempFile::new().unwrap();
    let f_other = NamedTempFile::new().unwrap();

    if o_base != "" {
        let content = data::get_object(o_base, "blob".to_owned());
        fs::write(f_base.path(), content).unwrap();
    }

    if o_head != "" {
        let content = data::get_object(o_head, "blob".to_owned());
        fs::write(f_head.path(), content).unwrap();
    }

    if o_other != "" {
        let content = data::get_object(o_other, "blob".to_owned());
        fs::write(f_other.path(), content).unwrap();
    }

    let output = Command::new("diff3")
        .arg("-m")
        .arg("-L")
        .arg("HEAD")
        .arg(f_head.path())
        .arg("-L")
        .arg("BASE")
        .arg(f_base.path())
        .arg("-L")
        .arg("MERGE_HEAD")
        .arg(f_other.path())
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to merge file");

    return String::from_utf8_lossy(&output.stdout).to_string();
}
