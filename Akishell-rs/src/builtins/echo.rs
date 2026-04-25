use std::ffi::OsString;

use crate::shell::context::ShellContext;
use crate::shell::error::ShellError;

pub fn run(_ctx: &mut ShellContext, args: &[OsString]) -> Result<i32, ShellError> {
    let mut newline = true;
    let mut parts = args;

    if let Some(first) = parts.first() {
        if first == "-n" {
            newline = false;
            parts = &parts[1..];
        }
    }

    let text = parts
        .iter()
        .map(|arg| arg.to_string_lossy())
        .collect::<Vec<_>>()
        .join(" ");

    if newline {
        println!("{text}");
    } else {
        print!("{text}");
    }

    Ok(0)
}
