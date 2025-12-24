use tree_sitter::Language;

pub fn language() -> Language {
    tree_sitter_python::LANGUAGE.into()
}

pub fn function_query(name: &str) -> String {
    format!(
        r#"(function_definition
            name: (identifier) @name
            (#eq? @name "{name}")
        ) @function"#
    )
}

pub fn class_query(name: &str) -> String {
    format!(
        r#"(class_definition
            name: (identifier) @name
            (#eq? @name "{name}")
        ) @class"#
    )
}

pub fn method_query(class_name: &str, method_name: &str) -> String {
    format!(
        r#"(class_definition
            name: (identifier) @class_name
            (#eq? @class_name "{class_name}")
            body: (block
                (function_definition
                    name: (identifier) @method_name
                    (#eq? @method_name "{method_name}")
                ) @method
            )
        )"#
    )
}

pub fn list_query() -> &'static str {
    r#"
    (function_definition name: (identifier) @func_name) @function
    (class_definition name: (identifier) @class_name) @class
    "#
}
