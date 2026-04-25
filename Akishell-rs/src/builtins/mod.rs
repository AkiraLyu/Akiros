mod cd;
mod echo;
mod env;
mod exit;
mod export;
mod pwd;
mod unset;

use std::collections::HashMap;
use std::ffi::OsString;

use super::shell::context::ShellContext;
use super::shell::error::ShellError;

pub type BuiltinFn = fn(&mut ShellContext, &[OsString]) -> Result<i32, ShellError>;

pub struct BuiltinRegistry {
    pub map: HashMap<&'static str, BuiltinFn>,
}

impl BuiltinRegistry {
    pub fn new() -> Self {
        Self {
            map: HashMap::from([
                ("cd", cd::run as BuiltinFn),
                ("pwd", pwd::run as BuiltinFn),
                ("echo", echo::run as BuiltinFn),
                ("export", export::run as BuiltinFn),
                ("unset", unset::run as BuiltinFn),
                ("exit", exit::run as BuiltinFn),
                ("env", env::run as BuiltinFn),
            ]),
        }
    }
}

impl Default for BuiltinRegistry {
    fn default() -> Self {
        Self::new()
    }
}
