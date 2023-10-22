use std::str;
use tree_sitter::Node;

pub fn str_from_bytes(bytes_slice: &[u8]) -> &str {
    std::str::from_utf8(bytes_slice).unwrap()
}

pub fn str_from_node<'a>(n: Node<'a>, source_code: &'a [u8]) -> &'a str {
    str_from_bytes(&source_code[n.byte_range()])
}

pub fn camel_to_snake(camel_case: &str) -> String {
    let mut snake_case = String::from("");
    let mut chars_iterator = camel_case.chars();

    // the first char is special: we do not add a `_` before
    if let Some(first_char) = chars_iterator.next() {
        if char::is_uppercase(first_char) {
            snake_case.push(first_char.to_ascii_lowercase());
        } else {
            snake_case.push(first_char);
        }
    }
    // then iterate over the rest
    for c in chars_iterator {
        if char::is_uppercase(c) {
            snake_case.push_str("_");
            snake_case.push(c.to_ascii_lowercase());
        } else {
            snake_case.push(c);
        }
    }
    snake_case
}
