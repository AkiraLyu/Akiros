use std::collections::HashMap;
use std::collections::HashSet;
use std::path::PathBuf;

pub struct ShellContext {
    pub vars: HashMap<String, String>,
    pub exported: HashSet<String>,
    pub last_status: i32,
    pub should_exit: bool,
    pub cwd: PathBuf,
}

impl ShellContext {
    pub fn new() -> Self {
        let vars: HashMap<String, String> = std::env::vars().collect();
        let exported = vars.keys().cloned().collect();
        let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));

        Self {
            vars,
            exported,
            last_status: 0,
            should_exit: false,
            cwd,
        }
    }
}
