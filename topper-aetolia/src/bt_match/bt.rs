use std::{fs, sync::OnceLock};

use crate::bt::LOAD_TREE_FUNC;

static BT_DIR: OnceLock<&'static str> = OnceLock::new();

fn load_tree_from_dir(tree_name: &String) -> String {
    let dir = BT_DIR.get().copied().unwrap_or("behavior_trees");
    let path = format!("{}/{}.json", dir, tree_name);
    fs::read_to_string(&path).unwrap_or_default()
}

/// Register the behavior-tree directory and install the tree loader.
pub fn set_bt_dir(dir: &str) {
    let leaked: &'static str = Box::leak(dir.to_owned().into_boxed_str());
    BT_DIR.set(leaked).ok();
    unsafe {
        LOAD_TREE_FUNC = Some(load_tree_from_dir);
    }
}
