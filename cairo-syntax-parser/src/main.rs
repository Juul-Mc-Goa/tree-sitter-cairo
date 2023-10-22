use std::str;

mod cairo_spec_parser;
mod lexer_file_parser;
mod parser_utils;

const AST_CODEGEN_FILE: &str = "src/cairo/crates/cairo-lang-syntax-codegen/src/cairo_spec.rs";
const LEXER_FILE: &str = "src/cairo/crates/cairo-lang-parser/src/lexer.rs";

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
    let hashmaps = lexer_file_parser::parse_lexer(LEXER_FILE);
    let part_grammar = cairo_spec_parser::parse_cairo_spec(AST_CODEGEN_FILE, hashmaps);
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
