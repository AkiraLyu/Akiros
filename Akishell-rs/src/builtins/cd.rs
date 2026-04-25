use std::ffi::OsString;
use std::path::PathBuf;

use crate::shell::context::ShellContext;
use crate::shell::error::ShellError;

pub fn run(ctx: &mut ShellContext, args: &[OsString]) -> Result<i32, ShellError> {
    if args.len() > 1 {
        return Err(ShellError::Builtin("cd: too many arguments".to_string()));
    }

    let target = match args.first() {
        Some(path) if path == "~" => home_dir(),
        Some(path) => PathBuf::from(path),
        None => home_dir(),
    };

    std::env::set_current_dir(&target)?;
    ctx.cwd = std::env::current_dir()?;
    ctx.vars
        .insert("PWD".to_string(), ctx.cwd.to_string_lossy().into_owned());

    Ok(0)
}

fn home_dir() -> PathBuf {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/"))
}
