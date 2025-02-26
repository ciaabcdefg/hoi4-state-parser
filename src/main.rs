use std::{env, fs, path::Path};
use typedefs::Statement;

mod json;
mod lazylexer;
mod lazyparser;
mod token;
mod typedefs;

fn process_input(file_path: &str) -> Result<String, String> {
    let file = fs::File::open(file_path).map_err(|_| "Failed to open file".to_string())?;
    let mut lexer = lazylexer::Lexer::new(file);
    let mut parser = lazyparser::Parser::new(&mut lexer);

    match parser
        .parse_program()
        .map_err(|_| "Failed to parse program".to_string())?
    {
        Statement::Assignment(assignment) => Ok(json::parse_expr_to_json(&assignment.value, 0)),
    }
}

fn main() {
    let mut args = env::args().skip(1);
    let mut in_file: Option<String> = None;
    let mut in_dir: Option<String> = None;
    let mut out_file: Option<String> = None;
    let mut out_dir: Option<String> = None;
    let mut out_echo = false;
    let mut echo_progress = true;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-i" | "--input-file" => in_file = args.next(),
            "-id" | "--input-directory" => in_dir = args.next(),
            "-e" | "--echo" => out_echo = true,
            "-s" | "--silence-progress" => echo_progress = false,
            "-o" | "--output" => out_file = args.next(),
            "-od" | "--output-directory" => out_dir = args.next(),
            _ => eprintln!("Unknown argument: {}", arg),
        }
    }

    if let Some(file_path) = in_file {
        if let Ok(json_output) = process_input(&file_path) {
            if out_echo {
                println!("{}", json_output);
            } else if let Some(out_path) = out_file {
                fs::write(out_path, json_output).expect("Failed to write output file");
            }
        }
    } else if let Some(dir_path) = in_dir {
        if let Ok(entries) = fs::read_dir(&dir_path) {
            for entry in entries.flatten() {
                let file_path = entry.path();
                if file_path.is_file() {
                    if echo_progress {
                        println!("Processing {:?}", file_path.file_name().unwrap());
                    }
                    if let Ok(json_output) = process_input(file_path.to_str().unwrap()) {
                        if out_echo {
                            println!("{}", json_output);
                        } else if let Some(out_dir) = &out_dir {
                            let out_path = Path::new(out_dir).join(file_path.file_name().unwrap());
                            fs::write(out_path, json_output).expect("Failed to write output file");
                        }
                    }
                }
            }
        } else {
            eprintln!("Failed to read input directory");
        }
    } else {
        eprintln!("Usage: -i <input_file_path> or -id <input_directory>");
    }
}
