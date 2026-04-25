use std::ffi::OsString;

use crate::shell::context::ShellContext;
use crate::shell::error::ShellError;

pub fn run(ctx: &mut ShellContext, args: &[OsString]) -> Result<i32, ShellError> {
    if args.len() > 1 {
        return Err(ShellError::Builtin("exit: too many arguments".to_string()));
    }

    let status = match args.first() {
        Some(value) => value
            .to_string_lossy()
            .parse::<i32>()
            .map_err(|_| ShellError::Builtin("exit: numeric argument required".to_string()))?,
        None => ctx.last_status,
    };

    ctx.last_status = status;
    ctx.should_exit = true;

    Ok(status)
}
