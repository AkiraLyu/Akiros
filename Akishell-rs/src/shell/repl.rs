use crate::builtins;
use std::ffi::OsString;

use super::context::ShellContext;
use super::error::ShellError;

pub fn run_repl(ctx: &mut ShellContext) -> Result<(), ShellError> {
    let builtins = builtins::BuiltinRegistry::new();

    loop {
        print!("Akishell$ ");
        std::io::Write::flush(&mut std::io::stdout())?;

        let mut line = String::new();
        if std::io::stdin().read_line(&mut line)? == 0 {
            break;
        }

        // builtin test
        let mut parts = line.split_whitespace();
        let Some(command) = parts.next() else {
            continue;
        };

        let args = parts.map(OsString::from).collect::<Vec<_>>();
        if let Some(builtin) = builtins.map.get(command) {
            match builtin(ctx, &args) {
                Ok(status) => ctx.last_status = status,
                Err(err) => {
                    ctx.last_status = err.exit_status();
                    eprintln!("{err}");
                }
            }
        } else {
            let err = ShellError::CommandNotFound(command.to_string());
            ctx.last_status = err.exit_status();
            eprintln!("{err}");
        }

        // let tokens = Lexer::new(&line).lex()?;
        // let ast = Parser::new(tokens).parse_program()?;

        // let status = Executor { ctx }.execute(&ast)?;
        // ctx.last_status = status;
        if ctx.should_exit {
            break;
        }
    }

    Ok(())
}
