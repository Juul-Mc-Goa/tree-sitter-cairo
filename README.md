# tree-sitter-cairo
A try at parsing Cairo's code (written in Rust) with tree-sitter, to produce a `grammar.js` for Cairo.

**WARNING: This project is not working yet, due to some ambiguities in the grammar file.**

## How to use
Type `cargo run > ../grammar.js` while in the directory `cairo-syntax-parser/`. 

## Overview of the project's logic

1. The whole AST for Cairo is defined [here](https://github.com/starkware-libs/cairo/blob/main/crates/cairo-lang-syntax-codegen/src/cairo_spec.rs).

2. There is also a lexer [there](https://github.com/starkware-libs/cairo/blob/main/crates/cairo-lang-parser/src/lexer.rs),
3. And more files [related to operator precedences](https://github.com/starkware-libs/cairo/blob/main/crates/cairo-lang-parser/src/operators.rs
) in the same project.
 As Cairo is defined with some Rust code, the idea is to programmatically parse some Rust using tree-sitter, in order to output a working `grammar.js` for Cairo. 
 
 ### Parsing `lexer.rs`
 
 There is one file for this: `cairo_syntax_parser/lexer_file_parser.rs`. It tries its best to automatically generate JS code from the `lexer.rs` file. In some cases, some hand-made JS code is written instead.
 This module has a main handle `parse_lexer(file: &str)` which returns two hashmaps for later use. These hashmaps are needed when parsing `cairo_spec.rs`, in order to replace some `SyntaxKind` with their `str` equivalent: for example, `SyntaxKind::TerminalComma` becomes `','`.
 
 ### Parsing `cairo_spec.rs`
 
 This is done by `cairo_syntax_parser/cairo_spec_parser.rs`. There is first a preprocessing phase to handle `add_option` method calls, and to remove structs that would match the empty string.
 Then the grammar is generated with a call to `iterate_nodes_aggregator`, which uses the two hashmaps from before.
