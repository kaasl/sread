pub mod python;
pub mod typescript;

use tree_sitter::{Language, Query};

pub trait LanguageSupport {
    fn language() -> Language;
    fn function_query() -> &'static str;
    fn class_query() -> &'static str;
    fn method_query() -> &'static str;
    fn list_query() -> &'static str;
}

pub fn get_query(lang: Language, query_str: &str) -> Result<Query, tree_sitter::QueryError> {
    Query::new(&lang, query_str)
}
