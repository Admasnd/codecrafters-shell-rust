#[allow(unused_imports)]
use std::io::{self, Result, Write};

fn main() -> io::Result<()> {
    loop {
        print!("$ ");
        io::stdout().flush()?;
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        println!("{}: command not found", buffer.trim());
    }
}
