use std::env;
use std::error::Error;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::ExitCode;
use std::result::Result;

fn print_prompt() -> io::Result<()> {
    print!("$ ");
    /* stdout usually line-buffered and print does not produce a new line
     * so we need to flush just in case
     */
    io::stdout().flush()?;
    Ok(())
}

fn read_input(buffer: &mut String) -> io::Result<()> {
    io::stdin().read_line(buffer)?;
    Ok(())
}

fn builtin_echo(text: &str) -> Result<Option<u8>, Box<dyn Error>> {
    println!("{}", text);
    Ok(None)
}

fn builtin_exit(arg: &str) -> Result<Option<u8>, Box<dyn Error>> {
    match arg.parse::<u8>() {
        Ok(exit_code) => Ok(Some(exit_code)),
        Err(_) => Ok(None),
    }
}

fn builtin_type(arg: &str) -> Result<Option<u8>, Box<dyn Error>> {
    let builtins = ["echo", "exit", "type"];
    if builtins.contains(&arg) {
        println!("{} is a shell builtin", arg);
        Ok(None)
    } else {
        // TODO use env::var(PATH) to check for executables
        println!("{}: not found", arg);
        Ok(None)
    }
}

fn eval_input(buffer: &str) -> Result<Option<u8>, Box<dyn Error>> {
    let cmd: Vec<_> = buffer.trim().splitn(2, ' ').collect();
    match &cmd[..] {
        ["echo", args] => builtin_echo(&args),
        ["exit", arg] => builtin_exit(&arg),
        ["type", arg] => builtin_type(&arg),
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
        let result = eval_input(&buffer)?;
        if let Some(exit_code) = result {
            return Ok(exit_code);
        }
        // Necessary because io::stdin().read_line(buffer) appends to buffer
        buffer.clear();
    }
}

fn main() -> ExitCode {
    match repl() {
        Ok(exit_code) => ExitCode::from(exit_code),
        Err(_) => ExitCode::FAILURE,
    }
}
