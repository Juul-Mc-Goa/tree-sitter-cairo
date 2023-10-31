use parser_utils::join_lines;
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

/// Stores how many spaces are inserted at the beginning of each line.
pub const LEADING_WHITESPACE: &str = "        ";

/// An `&str` containing the root SyntaxKind of a Cairo AST. It is used to place the corresponding
/// tree-sitter rule at the top of `grammar.js`.
const ROOT_NODE: &str = "SyntaxFile";
/// Which SyntaxKind to put binary operators precedence values in
const BINARY_EXPR_NODE: &str = "ExprBinary";
/// Which SyntaxKind to put unary operators precedence values in
const UNARY_EXPR_NODE: &str = "ExprUnary";

/// Which SyntaxKind to process specially in regards to list of nodes.
/// We need to wrap the arguments list with `optional(...)`.
const ALLOW_EMPTY_LIST: &[&str] = &["ArgListParenthesized", "FunctionSignature"];
/// Contains the Cairo tokens that should be ignored.
const TO_DELETE: &[&str] = &[
    "TokenSkipped",
    "TerminalEndOfFile",
    "BinaryOperator",
    "UnaryOperator",
];

fn conflicts() -> String {
    let result: Vec<String> = vec![
        "[$.expr_field_init_shorthand, $.path_segment_simple, $.path_segment_with_generic_args],"
            .to_string(),
        "[$.expr_field_init_shorthand, $.path_segment_simple],".to_string(),
        "[$.path_segment_simple, $.path_segment_with_generic_args],".to_string(),
        "[$.path_segment_simple, $.pattern_identifier],".to_string(),
        "[$.expr_parenthesized, $.expr_list_parenthesized],".to_string(),
        "[$.expr, $.expr_function_call],".to_string(),
        "[$.expr, $.expr_inline_macro],".to_string(),
        "[$.expr, $.expr_struct_ctor_call],".to_string(),
        "[$.expr_list, $.expr_parenthesized],".to_string(),
        "[$.pattern, $.pattern_enum],".to_string(),
    ];
    join_lines(result)
}
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
    let precedences = operators_parser::parse_operators(OPERATORS_FILE);
    let hashmaps = lexer_file_parser::parse_lexer(LEXER_FILE);
    let mut to_delete: HashSet<String> = HashSet::new();
    for token in TO_DELETE {
        to_delete.insert(String::from(*token));
    }
    let conflicts = conflicts();
    let part_grammar =
        cairo_spec_parser::parse_cairo_spec(AST_CODEGEN_FILE, hashmaps, precedences, to_delete);
    println!(
        "module.exports = grammar({{
    name: 'cairo',

    conflicts: $ => [
{conflicts}
    ],

    rules: {{
{part_grammar}
    }}
}});",
    );
}
