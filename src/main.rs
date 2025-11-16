use std::error::Error;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::ExitCode;
use std::result::Result;

fn print_prompt() -> io::Result<()> {
    print!("$ ");
    io::stdout().flush()?;
    Ok(())
}

fn read_input(buffer: &mut String) -> io::Result<()> {
    io::stdin().read_line(buffer)?;
    Ok(())
}

fn eval_input(buffer: &mut String) -> Result<Option<u8>, Box<dyn Error>> {
    let cmd: Vec<_> = buffer.trim().splitn(2, ' ').collect();
    match &cmd[..] {
        ["echo", args] => {
            println!("{}", args);
            Ok(None)
        }
        ["exit", n] => match n.parse::<u8>() {
            Ok(exit_code) => Ok(Some(exit_code)),
            Err(_) => Ok(None),
        },
        _ => {
            println!("{}: command not found", buffer.trim());
            Ok(None)
        }
    }
}

fn repl() -> Result<u8, Box<dyn Error>> {
    let mut buffer = String::new();
    loop {
        print_prompt()?;
        read_input(&mut buffer)?;
        let result = eval_input(&mut buffer)?;
        if let Some(exit_code) = result {
            return Ok(exit_code);
        }

        buffer.clear();
    }
}

fn main() -> ExitCode {
    match repl() {
        Ok(exit_code) => ExitCode::from(exit_code),
        Err(_) => ExitCode::FAILURE,
    }
}
