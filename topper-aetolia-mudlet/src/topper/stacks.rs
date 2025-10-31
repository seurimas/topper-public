use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
    sync::OnceLock,
};

use topper_aetolia::classes::{LOAD_STACK_FUNC, infiltrator::LOAD_HYPNO_STACK_FUNC};

static STACKS_DIRECTORY: OnceLock<&str> = OnceLock::new();

pub fn load_stack(class_name: &String, stack_name: &String) -> String {
    if let Ok(file) = unsafe {
        File::open(format!(
            "{}/{}/{}.json",
            STACKS_DIRECTORY.get().unwrap(),
            class_name,
            stack_name
        ))
    } {
        let mut reader = BufReader::new(file);
        let mut result = String::new();
        reader.read_to_string(&mut result);
        result
    } else {
        unsafe { format!("{}", STACKS_DIRECTORY.get().unwrap()) }
    }
}

pub fn initialize_load_stack_func(stacks_dir: String) {
    unsafe {
        STACKS_DIRECTORY.set(Box::leak(stacks_dir.into_boxed_str()));
        LOAD_STACK_FUNC = Some(load_stack);
        LOAD_HYPNO_STACK_FUNC = Some(load_stack);
    }
}
