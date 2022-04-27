use std::error::Error;
use std::io::{self, Write};
use std::collections::HashMap;
use simple_shell::parser::{Command, command};


fn main() -> Result<(), Box<dyn Error>> {

    let mut env : HashMap<_, _> = HashMap::new();

    loop {
        print!(">");
        io::stdout().flush()?;

        let mut command_line = String::new();

        io::stdin().read_line(&mut command_line)?;

        match command(&command_line) {
            Ok((_rest, com)) => {
                match com {
                    Command::Assignment(name,val) => { env.insert(name, val) }
                    Command::Empty => { continue }
                    Command::Quit => { return Ok(()) }
                }
            }
            Err(err) => {
                eprintln!("{}", err);
                break;
            }
        };

        println!("{}", command_line);
    }

    Ok(())
}
