use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
};

use typedefs::Statement;

mod json;
mod lazylexer;
mod lazyparser;
mod lexer;
mod parser;
mod token;
mod typedefs;

fn process_input(file_path: &String) -> Result<String, String> {
    let file = File::open(&file_path).expect("Failed to open file");
    let mut lexer = lazylexer::Lexer::new(file);
    let mut parser = lazyparser::Parser::new(&mut lexer);

    let program = parser.parse_program().unwrap();
    match program {
        Statement::Assignment(assignment) => {
            return Ok(json::parse_expr_to_json(&assignment.value, 0));
        }
    }
}

fn main() {
    let mut args = env::args().skip(1).peekable();
    let mut in_file_path: Option<String> = None;
    let mut in_dir: Option<String> = None;
    let mut out_file_path: Option<String> = None;
    let mut out_dir: Option<String> = None;
    let mut out_echo: bool = false;
    let mut echo_progress: bool = true;

    while let Some(arg) = args.next() {
        if arg == "-i" || arg == "--input-file" {
            in_file_path = args.next();
        } else if arg == "-id" || arg == "--input-directory" {
            in_dir = args.next();
        } else if arg == "-e" || arg == "--echo" {
            out_echo = true;
        } else if arg == "-s" || arg == "--silence-progress" {
            echo_progress = false;
        } else if arg == "-o" || arg == "--output" {
            if let Some(path) = args.next() {
                out_file_path = Some(path);
            } else {
                eprintln!("Usage: -o <output_file_path>");
                std::process::exit(1);
            }
        } else if arg == "-od" || arg == "--output-directory" {
            out_dir = args.next();
        }
    }

    if let Some(in_file_path) = in_file_path {
        let json_output = process_input(&in_file_path).unwrap();

        if out_echo {
            println!("{}", json_output);
        } else if let Some(out_file_path) = out_file_path {
            let mut out_file = File::create(out_file_path).unwrap();
            out_file.write_all(json_output.as_bytes()).unwrap();
        }
    } else if let Some(in_dir) = in_dir {
        let path = Path::new(&in_dir);
        if path.is_dir() {
            match path.read_dir() {
                Ok(entries) => {
                    for entry in entries.flatten() {
                        if let Ok(metadata) = entry.metadata() {
                            if metadata.is_file() {
                                let entry_path = &entry.path();
                                let file_name = entry_path.file_name().unwrap();

                                if echo_progress {
                                    println!("Processed {:?}", file_name);
                                }

                                let json_output = process_input(&entry_path.display().to_string());

                                if let Ok(json_output) = json_output {
                                    if out_echo {
                                        println!("{}", json_output);
                                    } else {
                                        let out_dir = out_dir.as_ref().unwrap();
                                        let out_file_path = Path::new(&out_dir).join(file_name);
                                        let mut out_file = File::create(out_file_path).unwrap();
                                        out_file.write_all(json_output.as_bytes()).unwrap();
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => eprintln!("Error reading directory {}", e),
            }
        }
    } else {
        eprintln!("Usage: -i <input_file_path>");
        std::process::exit(1);
    }
}
