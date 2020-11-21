use std::collections::HashMap;

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

pub fn diff_trees(t_from: HashMap<String, String>, t_to: HashMap<String, String>) -> String {
    let mut output = "".to_owned();
    let trees = vec![t_from, t_to];
    for (path, oids) in compare_trees(trees).iter() {
        let o_from = oids[0].clone();
        let o_to = oids[1].clone();
        if o_from != o_to {
            output.push_str(format!("changed {}\n", path).as_str());
        }
    }

    return output;
}
