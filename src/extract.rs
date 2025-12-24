use std::path::Path;
use streaming_iterator::StreamingIterator;
use tree_sitter::{Language, Parser, Query, QueryCursor};

use crate::languages::{python, typescript};

#[derive(Debug)]
pub enum SymbolType {
    Function,
    Class,
    Method { class: String },
    Interface,
}

#[derive(Debug)]
pub struct Symbol {
    pub name: String,
    pub kind: String,
}

pub fn detect_language(path: &Path) -> Option<Language> {
    match path.extension()?.to_str()? {
        "py" => Some(python::language()),
        "ts" | "mts" | "cts" => Some(typescript::language_typescript()),
        "tsx" => Some(typescript::language_tsx()),
        "js" | "mjs" | "cjs" | "jsx" => Some(typescript::language_typescript()),
        _ => None,
    }
}

pub fn extract_symbol(
    source: &str,
    path: &Path,
    symbol: &str,
) -> Result<String, String> {
    let lang = detect_language(path).ok_or("Unsupported file type")?;

    let mut parser = Parser::new();
    parser.set_language(&lang).map_err(|e| e.to_string())?;

    let tree = parser.parse(source, None).ok_or("Failed to parse")?;
    let root = tree.root_node();

    // Check for Class.method syntax
    if symbol.contains('.') {
        let parts: Vec<&str> = symbol.splitn(2, '.').collect();
        if parts.len() == 2 {
            return extract_method(source, path, &lang, root, parts[0], parts[1]);
        }
    }

    // Try function first, then class, then interface (for TS)
    if let Ok(result) = extract_by_type(source, path, &lang, root, symbol, SymbolType::Function) {
        return Ok(result);
    }
    if let Ok(result) = extract_by_type(source, path, &lang, root, symbol, SymbolType::Class) {
        return Ok(result);
    }
    if let Ok(result) = extract_by_type(source, path, &lang, root, symbol, SymbolType::Interface) {
        return Ok(result);
    }

    Err(format!("Symbol not found: {symbol}"))
}

fn extract_method(
    source: &str,
    path: &Path,
    lang: &Language,
    root: tree_sitter::Node,
    class_name: &str,
    method_name: &str,
) -> Result<String, String> {
    let query_str = if is_python(path) {
        python::method_query(class_name, method_name)
    } else {
        typescript::method_query(class_name, method_name)
    };

    let query = Query::new(lang, &query_str).map_err(|e| format!("Query error: {e}"))?;
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, root, source.as_bytes());

    while let Some(m) = matches.next() {
        for cap in m.captures {
            let name = query.capture_names()[cap.index as usize];
            if name == "method" {
                let node = cap.node;
                return Ok(source[node.byte_range()].to_string());
            }
        }
    }

    Err(format!("Method not found: {class_name}.{method_name}"))
}

fn extract_by_type(
    source: &str,
    path: &Path,
    lang: &Language,
    root: tree_sitter::Node,
    name: &str,
    sym_type: SymbolType,
) -> Result<String, String> {
    let query_str = match sym_type {
        SymbolType::Function => {
            if is_python(path) {
                python::function_query(name)
            } else {
                typescript::function_query(name)
            }
        }
        SymbolType::Class => {
            if is_python(path) {
                python::class_query(name)
            } else {
                typescript::class_query(name)
            }
        }
        SymbolType::Interface => {
            if is_python(path) {
                return Err("Python has no interfaces".to_string());
            }
            typescript::interface_query(name)
        }
        SymbolType::Method { .. } => return Err("Use extract_method".to_string()),
    };

    let query = Query::new(lang, &query_str).map_err(|e| format!("Query error: {e}"))?;
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, root, source.as_bytes());

    let capture_name = match sym_type {
        SymbolType::Function => "function",
        SymbolType::Class => "class",
        SymbolType::Interface => "interface",
        SymbolType::Method { .. } => unreachable!(),
    };

    while let Some(m) = matches.next() {
        for cap in m.captures {
            let cname = query.capture_names()[cap.index as usize];
            if cname == capture_name {
                let node = cap.node;
                return Ok(source[node.byte_range()].to_string());
            }
        }
    }

    Err(format!("{:?} not found: {name}", sym_type))
}

pub fn list_symbols(source: &str, path: &Path) -> Result<Vec<Symbol>, String> {
    let lang = detect_language(path).ok_or("Unsupported file type")?;

    let mut parser = Parser::new();
    parser.set_language(&lang).map_err(|e| e.to_string())?;

    let tree = parser.parse(source, None).ok_or("Failed to parse")?;
    let root = tree.root_node();

    let query_str = if is_python(path) {
        python::list_query()
    } else {
        typescript::list_query()
    };

    let query = Query::new(&lang, query_str).map_err(|e| format!("Query error: {e}"))?;
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, root, source.as_bytes());

    let mut symbols = Vec::new();
    let mut seen = std::collections::HashSet::new();

    while let Some(m) = matches.next() {
        for cap in m.captures {
            let cname = query.capture_names()[cap.index as usize];
            if cname.ends_with("_name") {
                let name = &source[cap.node.byte_range()];
                let kind = cname.trim_end_matches("_name");
                if seen.insert((name.to_string(), kind.to_string())) {
                    symbols.push(Symbol {
                        name: name.to_string(),
                        kind: kind.to_string(),
                    });
                }
            }
        }
    }

    Ok(symbols)
}

fn is_python(path: &Path) -> bool {
    path.extension().map(|e| e == "py").unwrap_or(false)
}
