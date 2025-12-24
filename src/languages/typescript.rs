use tree_sitter::Language;

pub fn language_typescript() -> Language {
    tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()
}

pub fn language_tsx() -> Language {
    tree_sitter_typescript::LANGUAGE_TSX.into()
}

// query to find a function by name 
// (both for classical function delarations and arrow functions)
pub fn function_query(name: &str) -> String {
    format!(
        r#"[
            (function_declaration
                name: (identifier) @name
                (#eq? @name "{name}")
            ) @function
            (lexical_declaration
                (variable_declarator
                    name: (identifier) @name
                    (#eq? @name "{name}")
                    value: (arrow_function)
                )
            ) @function
            (export_statement
                declaration: (function_declaration
                    name: (identifier) @name
                    (#eq? @name "{name}")
                )
            ) @function
        ]"#
    )
}

pub fn class_query(name: &str) -> String {
    format!(
        r#"[
            (class_declaration
                name: (type_identifier) @name
                (#eq? @name "{name}")
            ) @class
            (export_statement
                declaration: (class_declaration
                    name: (type_identifier) @name
                    (#eq? @name "{name}")
                )
            ) @class
        ]"#
    )
}

pub fn interface_query(name: &str) -> String {
    format!(
        r#"[
            (interface_declaration
                name: (type_identifier) @name
                (#eq? @name "{name}")
            ) @interface
            (export_statement
                declaration: (interface_declaration
                    name: (type_identifier) @name
                    (#eq? @name "{name}")
                )
            ) @interface
        ]"#
    )
}

pub fn method_query(class_name: &str, method_name: &str) -> String {
    format!(
        r#"(class_declaration
            name: (type_identifier) @class_name
            (#eq? @class_name "{class_name}")
            body: (class_body
                (method_definition
                    name: (property_identifier) @method_name
                    (#eq? @method_name "{method_name}")
                ) @method
            )
        )"#
    )
}

pub fn list_query() -> &'static str {
    r#"
    (function_declaration name: (identifier) @func_name) @function
    (class_declaration name: (type_identifier) @class_name) @class
    (interface_declaration name: (type_identifier) @interface_name) @interface
    (lexical_declaration (variable_declarator name: (identifier) @var_name value: (arrow_function))) @arrow_func
    "#
}
