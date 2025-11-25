// #[allow(unused_imports)]
use std::env;
use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
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

fn builtin_exit() -> Result<Option<u8>, Box<dyn Error>> {
    Ok(Some(0))
}

fn builtin_type(arg: &str) -> Result<Option<u8>, Box<dyn Error>> {
    let builtins = ["echo", "exit", "type"];
    let num_args = arg.split(' ').count();
    if num_args > 1 {
        println!("type given more than one argument: {}", arg);
        return Ok(None);
    }
    if builtins.contains(&arg) {
        println!("{} is a shell builtin", arg);
        Ok(None)
    } else {
        match env::var_os("PATH") {
            Some(paths) => {
                for path in env::split_paths(&paths) {
                    // check if executable exists
                    let exec_path = format!("{}/{}", path.display(), arg);
                    let does_exists = fs::exists(&exec_path)?;
                    if !does_exists {
                        continue;
                    }
                    // check if has execute permission
                    let perms = fs::metadata(&exec_path)?.permissions().mode();
                    /* 0o represents octal notation where each digit is 0-7.
                     * Each octal digit represents a permission.
                     * - 4 = read
                     * - 2 = write
                     * - 1 = execute
                     * The first digit represents the owner permission.
                     * The second digit represents the group permission.
                     * The third digit represents the other permission.
                     * Using bitwise and `&` with 0o111 isolates the write permissions by clearing
                     * the other permission bits (i.e., setting them to 0).
                     */
                    let can_exec = (perms & 0o111) != 0;
                    if !can_exec {
                        continue;
                    }
                    // executable found
                    println!("{} is {}", arg, exec_path);
                    return Ok(None);
                }
                // executable not found in any path
                println!("{}: not found", arg);
                Ok(None)
            }
            None => {
                // PATH not set so no executable can be found
                println!("{}: not found", arg);
                Ok(None)
            }
        }
    }
}

fn eval_input(buffer: &str) -> Result<Option<u8>, Box<dyn Error>> {
    let cmd: Vec<_> = buffer.trim().splitn(2, ' ').collect();
    match &cmd[..] {
        ["echo", args] => builtin_echo(&args),
        ["exit"] => builtin_exit(),
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
