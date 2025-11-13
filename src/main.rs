use std::error::Error;
#[allow(unused_imports)]
use std::io::{self, Write};
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

fn eval_input(buffer: &mut String) -> Result<(), Box<dyn Error>> {
    let cmd: Vec<_> = buffer.trim().splitn(2, ' ').collect();
    match &cmd[..] {
        ["exit", n] if n.parse::<u64>().is_ok() => Err(Box::<dyn Error>::from("")),
        _ => {
            println!("{}: command not found", buffer.trim());
            Ok(())
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    loop {
        print_prompt()?;
        read_input(&mut buffer)?;
        eval_input(&mut buffer)?;
        buffer.clear();
    }
}
