use tree_sitter::Language;

pub fn language() -> Language {
    tree_sitter_rust::LANGUAGE.into()
}

pub fn function_query(name: &str) -> String {
    format!(
        r#"(function_item
            name: (identifier) @name
            (#eq? @name "{name}")
        ) @function"#
    )
}

// rust has structs, not classes so we'll use structs
pub fn class_query(name: &str) -> String {
    format!(
        r#"[
            (struct_item
                name: (type_identifier) @name
                (#eq? @name "{name}")
            ) @class
            (enum_item
                name: (type_identifier) @name
                (#eq? @name "{name}")
            ) @class
        ]"#
    )
}

pub fn trait_query(name: &str) -> String {
    format!(
        r#"(trait_item
            name: (type_identifier) @name
            (#eq? @name "{name}")
        ) @trait"#
    )
}

pub fn method_query(type_name: &str, method_name: &str) -> String {
    format!(
        r#"(impl_item
            type: (type_identifier) @type_name
            (#eq? @type_name "{type_name}")
            body: (declaration_list
                (function_item
                    name: (identifier) @method_name
                    (#eq? @method_name "{method_name}")
                ) @method
            )
        )"#
    )
}

pub fn list_query() -> &'static str {
    r#"
    (function_item name: (identifier) @func_name) @function
    (struct_item name: (type_identifier) @struct_name) @struct
    (enum_item name: (type_identifier) @enum_name) @enum
    (trait_item name: (type_identifier) @trait_name) @trait
    (impl_item type: (type_identifier) @impl_name) @impl
    (mod_item name: (identifier) @mod_name) @mod
    "#
}
