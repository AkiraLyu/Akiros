use std::ffi::OsString;

use crate::shell::context::ShellContext;
use crate::shell::error::ShellError;

pub fn run(ctx: &mut ShellContext, args: &[OsString]) -> Result<i32, ShellError> {
    if args.is_empty() {
        let mut names = ctx.exported.iter().collect::<Vec<_>>();
        names.sort();

        for name in names {
            if let Some(value) = ctx.vars.get(name) {
                println!("export {name}=\"{value}\"");
            }
        }

        return Ok(0);
    }

    for arg in args {
        let assignment = arg.to_string_lossy();
        let Some((name, value)) = assignment.split_once('=') else {
            ctx.exported.insert(assignment.into_owned());
            continue;
        };

        ctx.vars.insert(name.to_string(), value.to_string());
        ctx.exported.insert(name.to_string());
    }

    Ok(0)
}
