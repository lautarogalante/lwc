use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const PROGRAM_NAME: &str = "lwc";

struct CommandInfo {
    action: Option<fn(&str) -> Result<(), Box<dyn Error>>>,
    help: Option<fn() -> Result<(), Box<dyn Error>>>,
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
    let file = File::open(path).expect("failed to open file.");
    let reader = BufReader::new(file);
    let count = reader.lines().count();

    println!("{} {}", count, get_file_name(path));
    Ok(())
}
fn get_words(path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(path).expect("failed to open file.");
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
    let file = File::open(path).expect("failed.");
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
fn init(command: &str, path: &str) -> Result<(), Box<dyn Error>> {
    let mut arguments: HashMap<String, CommandInfo> = HashMap::new();
    arguments.insert(
        "-h".to_string(),
        CommandInfo {
            action: None,
            help: Some(get_help),
        },
    );
    arguments.insert(
        "-c".to_string(),
        CommandInfo {
            action: Some(get_bytes),
            help: None,
        },
    );
    arguments.insert(
        "-l".to_string(),
        CommandInfo {
            action: Some(get_lines),
            help: None,
        },
    );
    arguments.insert(
        "-w".to_string(),
        CommandInfo {
            action: Some(get_words),
            help: None,
        },
    );
    arguments.insert(
        "-m".to_string(),
        CommandInfo {
            action: Some(get_chars),
            help: None,
        },
    );

    if let Some(cmd_op) = arguments.get(command) {
        if let Some(action) = cmd_op.action {
            action(path)?;
        }
        if let Some(help) = cmd_op.help {
            let _ = help();
        }
    } else {
        println!("unrecognized argument: {}", command)
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let command = args.get(1).map(|s| s.as_str()).unwrap_or("");
        let path = args.get(2).map(|s| s.as_str()).unwrap_or("");
        let _ = init(command, path)?;
    }
    Ok(())
}
