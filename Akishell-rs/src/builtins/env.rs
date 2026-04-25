use std::ffi::OsString;

use crate::shell::context::ShellContext;
use crate::shell::error::ShellError;

pub fn run(ctx: &mut ShellContext, _args: &[OsString]) -> Result<i32, ShellError> {
    let mut names = ctx.exported.iter().collect::<Vec<_>>();
    names.sort();

    for name in names {
        if let Some(value) = ctx.vars.get(name) {
            println!("{name}={value}");
        }
    }

    Ok(0)
}
