use std::env;
use std::fs;
use std::io::{self, Write};

mod tokenize;
use tokenize::tokenize;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            let (tokens, token_errors) = tokenize(file_contents);

            // imprimiendo en la salida estandar los tokens
            let tokens_msg = tokens
                .iter()
                .map(|token| token.to_string())
                .collect::<Vec<String>>()
                .join("\n");
            println!("{}", tokens_msg);

            // imprimiendo en stderr los errores
            let token_errors_msg = token_errors
                .iter()
                .map(|token_error| token_error.to_string())
                .collect::<Vec<String>>()
                .join("\n");

            if !token_errors_msg.is_empty() {
                eprintln!("{}", token_errors_msg);
            }

        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
