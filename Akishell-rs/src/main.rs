mod shell;
mod expand;
mod exec;
mod builtins;
mod syntax;

use shell::error::ShellError;
use shell::context::ShellContext;

fn main() -> Result<(), ShellError> {
    let mut ctx = ShellContext::new();
    shell::repl::run_repl(&mut ctx)?;
    Ok(())
}
