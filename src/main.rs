// #[allow(unused_imports)]
use anyhow::anyhow;
use anyhow::Result;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::process::{Command, ExitCode};

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

fn builtin_echo(text: &str) -> Result<Option<u8>> {
    println!("{}", text);
    Ok(None)
}

fn builtin_exit() -> Result<Option<u8>> {
    Ok(Some(0))
}

fn get_exec_path(arg: &str) -> Result<String> {
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
                return Ok(path.display().to_string());
            }
            // executable not found in any path
            Err(anyhow!("{}: not found", arg))
        }
        None => {
            // PATH not set so no executable can be found
            Err(anyhow!("{}: not found", arg))
        }
    }
}

fn builtin_type(args: &[&str]) -> Result<Option<u8>> {
    let builtins = ["echo", "exit", "type"];
    match &args[..] {
        [arg] => {
            if builtins.contains(arg) {
                println!("{} is a shell builtin", arg);
                Ok(None)
            } else {
                match get_exec_path(&arg) {
                    Ok(path) => {
                        println!("{} is {}", arg, path);
                        Ok(None)
                    }
                    Err(err) => {
                        println!("{err}");
                        Ok(None)
                    }
                }
            }
        }
        _ => {
            println!("type: invalid number of arguments");
            Ok(None)
        }
    }
}

fn exec_program(cmd: &str, args: &[&str]) -> Result<Option<u8>> {
    let output = Command::new(&cmd).args(args).output()?;
    io::stdout().write_all(&output.stdout)?;
    Ok(None)
}

fn eval_input(buffer: &str) -> Result<Option<u8>> {
    let mut args = buffer.trim().split(' ');
    match args.next() {
        Some("echo") => builtin_echo(&args.collect::<Vec<_>>().join(" ")),
        Some("exit") => builtin_exit(),
        Some("type") => builtin_type(&args.collect::<Vec<_>>()),
        Some(cmd) => match get_exec_path(cmd) {
            Ok(path) => {
                let exec_path = format!("{}/{}", path, cmd);
                exec_program(&exec_path, &args.collect::<Vec<_>>())
            }
            Err(_) => {
                println!("{}: command not found", buffer.trim());
                Ok(None)
            }
        },
        _ => {
            println!("{}: command not found", buffer.trim());
            Ok(None)
        }
    }
}

fn repl() -> Result<u8> {
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
