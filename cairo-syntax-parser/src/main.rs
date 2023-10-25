use std::{collections::HashSet, str};

mod cairo_spec_parser;
mod lexer_file_parser;
mod operators_parser;
mod parser_utils;

/// path to `cairo_spec.rs`
const AST_CODEGEN_FILE: &str = "src/cairo/crates/cairo-lang-syntax-codegen/src/cairo_spec.rs";
/// path to `lexer_file.rs`
const LEXER_FILE: &str = "src/cairo/crates/cairo-lang-parser/src/lexer.rs";
/// path to `operators.rs`
const OPERATORS_FILE: &str = "src/cairo/crates/cairo-lang-parser/src/operators.rs";

/// Contains the Cairo tokens that should be ignored.
const TO_DELETE: &[&str] = &["TokenSkipped", "TerminalEndOfFile"];

/// An `&str` containing the root SyntaxKind of a Cairo AST. It is used to place the corresponding
/// tree-sitter rule at the top of `grammar.js`.
const ROOT_NODE: &str = "SyntaxFile";

/// Stores how many spaces are inserted at the beginning of each line.
pub const LEADING_WHITESPACE: &str = "        ";

/// The whole syntax tree is in `AST_CODEGEN_FILE`, one only needs to map some `SyntaxKind`
/// to their actual `char` representation (ie `SyntaxKind::TerminalLBrack => '['`).
/// For that there are, in `LEXER_FILE`:
/// 1. `token_kind_to_terminal_syntax_kind(...)`,
/// 2. `match_terminal(...)`.
/// Parsing 1. produces a keymap `TokenKind -> SyntaxKind` that one can inverse.
/// Parsing 2. produces a keymap `String -> TokenKind` that one can inverse.
/// The idea is then to produce a map `SyntaxKind -> String`, and use it to replace the
/// appropriate nodes in `AST_CODEGEN_FILE`; in order to obtain a working `grammar.js`.
fn main() {
    let operator_precedence = operators_parser::parse_operators(OPERATORS_FILE);
    let hashmaps = lexer_file_parser::parse_lexer(LEXER_FILE);
    let mut to_delete: HashSet<String> = HashSet::new();
    for token in TO_DELETE {
        to_delete.insert(String::from(*token));
    }
    let part_grammar = cairo_spec_parser::parse_cairo_spec(AST_CODEGEN_FILE, hashmaps, to_delete);
    println!(
        "module.exports = grammar({{
    name: 'cairo',

    rules: {{
{}
    }}
}});",
        part_grammar
    );
}
