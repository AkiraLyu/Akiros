use std::ffi::OsString;

use crate::shell::context::ShellContext;
use crate::shell::error::ShellError;

pub fn run(ctx: &mut ShellContext, args: &[OsString]) -> Result<i32, ShellError> {
    for arg in args {
        let name = arg.to_string_lossy();
        ctx.vars.remove(name.as_ref());
        ctx.exported.remove(name.as_ref());
    }

    Ok(0)
}
