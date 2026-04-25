use std::ffi::OsString;

use crate::shell::context::ShellContext;
use crate::shell::error::ShellError;

pub fn run(ctx: &mut ShellContext, _args: &[OsString]) -> Result<i32, ShellError> {
    println!("{}", ctx.cwd.display());
    Ok(0)
}
