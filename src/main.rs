mod extract;
mod languages;

use std::fs;
use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Usage: sread <file>:<symbol>");
        eprintln!("       sread <file>:<type>:<name>");
        eprintln!("       sread <file> --list");
        return ExitCode::from(2);
    }

    if args.len() == 2 && args[1] == "--list" {
        return list_symbols(&args[0]);
    }
    
    let input = &args[0];
    let (file_path, symbol) = match parse_input(input) {
        Some(v) => v,
        None => {
            eprintln!("Invalid format. Use <file>:<symbol> or <file>:<type>:<name>");
            return ExitCode::from(2);
        }
    };

    extract_and_print(&file_path, &symbol)
}

fn parse_input(input: &str) -> Option<(String, String)> {
    // find the last colon that separates file from symbol
    // handle Windows paths (C:\...) by looking for pattern after extension
    let extensions = [".py:", ".ts:", ".tsx:", ".js:", ".jsx:", ".mts:", ".cts:", ".mjs:", ".cjs:"];

    for ext in extensions {
        if let Some(pos) = input.find(ext) {
            let split_pos = pos + ext.len() - 1; // position of the colon
            let file = &input[..split_pos];
            let symbol = &input[split_pos + 1..];
            if !symbol.is_empty() {
                return Some((file.to_string(), symbol.to_string()));
            }
        }
    }

    None
}

fn extract_and_print(file_path: &str, symbol: &str) -> ExitCode {
    let path = Path::new(file_path);

    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading file: {e}");
            return ExitCode::from(2);
        }
    };

    let symbol = if let Some((type_prefix, name)) = symbol.split_once(':') {
        match type_prefix {
            "function" | "func" | "fn" => name.to_string(),
            "class" => name.to_string(),
            "method" => name.to_string(),
            "interface" => name.to_string(),
            _ => symbol.to_string(),
        }
    } else {
        symbol.to_string()
    };

    match extract::extract_symbol(&source, path, &symbol) {
        Ok(code) => {
            print!("{code}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{e}");
            ExitCode::from(1)
        }
    }
}

fn list_symbols(file_path: &str) -> ExitCode {
    let path = Path::new(file_path);

    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading file: {e}");
            return ExitCode::from(2);
        }
    };

    match extract::list_symbols(&source, path) {
        Ok(symbols) => {
            for sym in symbols {
                println!("{}: {}", sym.kind, sym.name);
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{e}");
            ExitCode::from(2)
        }
    }
}
