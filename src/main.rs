use std::env::Args;
use std::io::{self, Bytes, Read};

fn main() {
    let mut args = std::env::args();
    let program_name = args.next().unwrap_or("aash".to_string());

    let result = match args.next() {
        Some(arg) if arg == "-c" => handle_command_flag(args, &program_name),
        Some(filename) => handle_file_input(&filename, &program_name),
        None => parse_execute_loop(io::stdin().bytes()),
    };

    if let Err(err) = result {
        eprintln!("{program_name}: error: {err}");
        std::process::exit(1);
    }
}

fn handle_command_flag(mut args: Args, program_name: &str) -> io::Result<()> {
    Ok(()) // TODO
}

fn handle_file_input(filename: &str, program_name: &str) -> io::Result<()> {
    Ok(()) // TODO
}

fn parse_execute_loop<R: Read>(stream: Bytes<R>) -> io::Result<()> {
    Ok(()) // TODO
}
