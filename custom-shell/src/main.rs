/* 
    Author: José Areia
    Date: 31/10/2024
    Reference: https://www.joshmcguigan.com/blog/build-your-own-shell-rust
*/

use std::env;
use std::io::Write;
use std::io::stdin;
use std::io::stdout;
use std::path::Path;
use std::process::Stdio;
use std::process::Child;
use std::process::Command;

fn main(){
    loop {
        print!("> ");
        stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // Must be peekable so we know when we are on the last command.
        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next()  {

            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "cd" => {
                    let new_dir = args.peekable().peek()
                        .map_or("/", |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }

                    previous_command = None;
                },
                "exit" => return,
                command => {
                    let stdin = previous_command
                        .map_or(
                            Stdio::inherit(),
                            |output: Child| Stdio::from(output.stdout.unwrap())
                        );

                    let stdout = if commands.peek().is_some() {
                        // There is another command piped behind this one prepare to send output to the next command.
                        Stdio::piped()
                    } else {
                        // There are no more commands piped behind this one send output to shell stdout.
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(output) => { previous_command = Some(output); },
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        },
                    };
                }
            }
        }

        if let Some(mut final_command) = previous_command {
            // Block until the final command has finished.
            final_command.wait().expect("Failed to flush stdout");
        }
    }
}