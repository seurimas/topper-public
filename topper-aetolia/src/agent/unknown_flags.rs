use std::{collections::HashSet, hash::Hash, sync::Arc};

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct UnknownFlagSet {
    pub set_flags: HashSet<Arc<str>>,
}

impl Hash for UnknownFlagSet {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for flag in &self.set_flags {
            flag.hash(state);
        }
    }
}

impl UnknownFlagSet {
    pub fn add_flag(&mut self, flag: &str) {
        self.set_flags.insert(Arc::from(flag));
    }

    pub fn remove_flag(&mut self, flag: &str) {
        self.set_flags.remove(flag);
    }

    pub fn has_flag(&self, flag: &str) -> bool {
        self.set_flags.contains(flag)
    }

    pub fn clear(&mut self) {
        self.set_flags.clear();
    }
}
