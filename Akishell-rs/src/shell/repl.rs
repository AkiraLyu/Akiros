use super::context::ShellContext;
use super::error::ShellError;

pub fn run_repl(ctx: &mut ShellContext) -> Result<(), ShellError> {
    loop {
        print!("Akishell$ ");
        std::io::Write::flush(&mut std::io::stdout())?;

        let mut line = String::new();
        if std::io::stdin().read_line(&mut line)? == 0 {
            break;
        }
        // println!("{}",line);

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
