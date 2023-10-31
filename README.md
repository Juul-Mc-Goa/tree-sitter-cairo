# tree-sitter-cairo
An attempt at parsing Cairo's code (written in Rust) with tree-sitter, to produce a `grammar.js` for Cairo.

**WARNING: This project is not working yet, due to some ambiguities in the grammar file.**

## How to use
Make sure you have `tree-sitter, nodejs` installed.
``` sh
cd cairo-syntax-parser/
cargo run > ../grammar.js
cd ../
tree-sitter generate
```


## Overview of the project's logic

1. The whole syntax for Cairo is defined [here](https://github.com/starkware-libs/cairo/blob/main/crates/cairo-lang-syntax-codegen/src/cairo_spec.rs).

2. There is also a lexer [there](https://github.com/starkware-libs/cairo/blob/main/crates/cairo-lang-parser/src/lexer.rs),
3. And more files [related to operators precedence](https://github.com/starkware-libs/cairo/blob/main/crates/cairo-lang-parser/src/operators.rs
) in the same project.

 As Cairo is defined with some Rust code, the idea is to programmatically parse some Rust using tree-sitter, in order to output a working `grammar.js` for Cairo. 
 
 This code first parses `lexer.rs` and `operators.rs`, then parses `cairo_spec.rs` and generate the grammar.

### Parsing `lexer.rs`
 
 This is done by: `cairo_syntax_parser/lexer_file_parser.rs`. 
 
 It tries its best to automatically generate JS code from the `lexer.rs` file. In some cases, some hand-made JS code is written instead.

This module has a main handle `parse_lexer(file: &str)` which returns two hashmaps for later use.

### Parsing `operators.rs`
 This is done by: `cairo_syntax_parser/operators_parser.rs`. It produces two hashmaps `unary_precedence, post_precedence`, both of type `HashMap<u32, Vec<String>>`. Each maps one level of precedence `k` to a list of those operators who have precedence equal to `k`.

This module has a main handle `parse_operators(file: &str)` which returns two hashmaps for later use.
 
### Parsing `cairo_spec.rs`
 
 This is done by `cairo_syntax_parser/cairo_spec_parser.rs`.
 
 There is first a preprocessing phase to handle `add_option` method calls, and to remove structs that would match the empty string.
 Then the grammar is generated with a call to `iterate_nodes_aggregator`, which uses the hashmaps from before.
