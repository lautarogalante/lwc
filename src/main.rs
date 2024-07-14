use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const PROGRAM_NAME: &str = "lwc";

struct CommandInfo {
    action: Option<fn(&str) -> Result<(), Box<dyn Error>>>,
}

fn get_help() -> Result<(), Box<dyn Error>> {
    let help_lines = vec![
        format!("Usage: {} [option][file name]", PROGRAM_NAME),
        "".to_string(),
        format!(
            "{} -c [file name] Outputs the number of bytes in a file.",
            PROGRAM_NAME
        ),
        format!(
            "{} -l [file name] Outputs the number of lines in a file.",
            PROGRAM_NAME
        ),
        format!(
            "{} -w [file name] Outputs the number of words in a file.",
            PROGRAM_NAME
        ),
        format!(
            "{} -m [file name] Outputs the number of characters in a file.",
            PROGRAM_NAME
        ),
    ];

    println!("{}", help_lines.join("\n"));
    Ok(())
}

fn get_bytes(path: &str) -> Result<(), Box<dyn Error>> {
    let metadata = std::fs::metadata(path)?;
    let bytes = metadata.len();
    println!("{} {}", bytes, get_file_name(path));
    Ok(())
}

fn get_lines(path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let count = reader.lines().count();

    println!("{} {}", count, get_file_name(path));
    Ok(())
}
fn get_words(path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut words_counts = 0;
    for line in reader.lines() {
        let line = line?;
        words_counts += line.split_whitespace().count();
    }
    println!("{} {}", words_counts, get_file_name(path));
    Ok(())
}

fn get_chars(path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut count = 0;
    for line in reader.lines() {
        for _c in line.expect("Lines failed").chars() {
            count += 1;
        }
    }
    println!("{} {}", count, get_file_name(path));
    Ok(())
}

fn get_file_name(path: &str) -> String {
    let file_name = Path::new(path)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy();
    return file_name.to_string();
}

fn all_functions(path: &str) -> Result<(), Box<dyn Error>> {
    get_bytes(path)?;
    get_lines(path)?;
    get_words(path)?;
    get_chars(path)?;
    Ok(())
}

fn init(mut args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut arguments: HashMap<String, CommandInfo> = HashMap::new();
    arguments.insert("-h".to_string(), CommandInfo { action: None });
    arguments.insert(
        "-c".to_string(),
        CommandInfo {
            action: Some(get_bytes),
        },
    );
    arguments.insert(
        "-l".to_string(),
        CommandInfo {
            action: Some(get_lines),
        },
    );
    arguments.insert(
        "-w".to_string(),
        CommandInfo {
            action: Some(get_words),
        },
    );
    arguments.insert(
        "-m".to_string(),
        CommandInfo {
            action: Some(get_chars),
        },
    );

    let path = args.pop();
    for flag in args.iter() {
        if let Some(cmd_op) = arguments.get(flag) {
            if let Some(action) = cmd_op.action {
                if let Some(path_arg) = &path {
                    action(path_arg)?;
                } else {
                    println!("not path provided.");
                }
            }
        } else {
            if flag != PROGRAM_NAME {
                println!("unrecognized argument: {}", flag);
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.get(1).map_or(false, |s| s.contains("-h")) || args.len() == 1 {
        get_help()?;
    } else if args.len() > 1 && args.get(1).map_or(false, |s| s.contains("-")) {
        init(args)?;
    } else {
        let path = args.get(1).map(|s| s.as_str()).unwrap_or("");
        all_functions(path)?;
    }
    Ok(())
}
