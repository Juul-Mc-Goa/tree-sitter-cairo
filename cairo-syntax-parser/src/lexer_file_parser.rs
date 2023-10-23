use std::{collections::HashMap, fs, str};
use tree_sitter::{Node, Parser, Query, QueryCursor, TreeCursor};

const TYPE_SUFFIX: &str = "type_suffix: _ => /_[0-9a-zA-Z]+/,\n";
const ESCAPE_SEQUENCE: &str = "escape_sequence: _ => /\\\\./,\n";
const TOKEN_SINGLE_LINE_COMMENT: &str = "token_single_line_comment: _ => /\\/\\/.*$/,\n";
const TOKEN_NEWLINE: &str = "token_newline: _ => /\\n/,\n";
const TOKEN_WHITESPACE: &str = "token_whitespace: _ => /[ \\r\\t]/,\n";
// const TOKEN_SKIPPED: &str = "token_skipped: _ => /$^/,\n";
// const TERMINAL_EOF: &str = "terminal_end_of_file: _ => /\\z/,\n";

fn build_initial_value() -> String {
    let lines = vec![
        TYPE_SUFFIX,
        ESCAPE_SEQUENCE,
        TOKEN_SINGLE_LINE_COMMENT,
        TOKEN_NEWLINE,
        TOKEN_WHITESPACE,
        // TOKEN_SKIPPED,
        // TERMINAL_EOF,
    ];
    lines.join("\n")
}

#[derive(Clone)]
struct LexerFileParser<'a> {
    cursor: TreeCursor<'a>,
    source_code: &'a [u8],
    language_var: tree_sitter::Language,
    pub token_to_str: HashMap<String, String>,
    pub kind_to_token: HashMap<String, String>,
}

impl<'a> LexerFileParser<'a> {
    pub fn new(
        cursor: TreeCursor<'a>,
        source_code: &'a [u8],
        language_var: tree_sitter::Language,
    ) -> Self {
        let mut lexer_file_parser = LexerFileParser {
            cursor,
            source_code,
            language_var,
            token_to_str: HashMap::<String, String>::new(),
            kind_to_token: HashMap::<String, String>::new(),
        };

        lexer_file_parser
            .token_to_str
            .insert(String::from(""), build_initial_value());
        lexer_file_parser
    }

    /// helper function to get the code represented by a given Node
    fn str_from_node(&self, n: Node<'a>) -> &'a str {
        std::str::from_utf8(&self.source_code[n.byte_range()]).unwrap()
    }

    fn token_to_str_push_value(mut self, key: String, to_push: String) -> Self {
        let mut value = self.token_to_str.get("").unwrap().clone();
        value.push_str(&to_push);
        let _ = self.token_to_str.insert(key, value);
        self
    }

    /// iterate over arguments of a call expression
    pub fn iterate_arguments(&mut self, call_node: Node<'a>) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let mut new_cursor = self.cursor.clone();
        for child in call_node.children(&mut new_cursor) {
            match self.str_from_node(child) {
                "," => (),
                "(" => (),
                ")" => (),
                arg => result.push(String::from(arg.trim_matches('\''))),
            }
        }
        result
    }

    /// this one is copy-pasted from rust's grammar.js
    pub fn take_token_literal_number(self) -> Self {
        let to_push = "
terminal_literal_number: $ => token(seq(
    choice(
      /[0-9][0-9_]*/,
      /0x[0-9a-fA-F_]+/,
      /0b[01_]+/,
      /0o[0-7_]+/,
    ),
    optional(/_[0-9a-zA-Z]+/),
)),
";
        self.token_to_str_push_value("".into(), to_push.into())
    }

    /// hand-made premium tree-sitter rule
    pub fn take_token_string(self) -> Self {
        let to_push = "
terminal_string: $ => seq(
    '\"',
    repeat(choice($.escape_sequence, /[^\"]/)),
    '\"',
    optional($.type_suffix),
),
";
        self.token_to_str_push_value("".into(), to_push.into())
    }

    /// hand-made premium tree-sitter rule
    pub fn take_token_short_string(self) -> Self {
        let to_push = "
terminal_short_string: $ => seq(
    '\\\'',
    repeat(choice($.escape_sequence, /[^\']/)),
    '\\\'',
    optional($.type_suffix),
),
";
        self.token_to_str_push_value("".into(), to_push.into())
    }

    pub fn pick_kind(mut self, pattern: Node, args: Node<'a>) -> Self {
        let base_pattern = self.str_from_node(pattern).trim_matches('\'');
        let args_vec: Vec<String> = self.iterate_arguments(args);

        let mut big_pattern = String::from(base_pattern);
        big_pattern.push_str(args_vec[0].trim_matches('\''));
        let big_kind = &args_vec[1];
        let base_kind = &args_vec[2];
        let _ = self
            .token_to_str
            .insert(base_kind.into(), format!("'{base_pattern}'"));
        let _ = self
            .token_to_str
            .insert(big_kind.into(), format!("'{big_pattern}'"));
        self
    }

    /// takes `pattern => self.take_token_of_kind(KIND)`, updates `self.token_to_str` such that
    /// `self.token_to_str.get(KIND) == pattern`
    pub fn take_token_of_kind(mut self, pattern: Node, args: Node<'a>) -> Self {
        let value = self.str_from_node(pattern); // keep surrounding quotes
        let key = &self.iterate_arguments(args)[0];
        let _ = self.token_to_str.insert(key.into(), value.into());
        self
    }

    pub fn take_token_identifier(self) -> Self {
        let to_push = "
terminal_identifier: _ => /[a-zA-Z_][a-zA-Z0-9_]*/,
";
        self.token_to_str_push_value("".into(), to_push.into())
    }

    pub fn parse_token_kind_to_syntax_kind(mut self, root_node: Node) -> Self {
        // captures all `match` arms of the function `token_kind_to_terminal_syntax_kind`
        let query_token_to_syntax = "
        (function_item
            name: (identifier) @fn_name
            body: (block
                    (expression_statement
                      (match_expression
                          body: (match_block
                                    (match_arm pattern: (match_pattern) @token_kind
                                               value: (scoped_identifier) @syntax_kind)))))
        (#eq? @fn_name \"token_kind_to_terminal_syntax_kind\"))";

        let query2 = Query::new(self.language_var, query_token_to_syntax).unwrap();
        let mut query_cursor2 = QueryCursor::new();
        let query2_matches = query_cursor2.matches(&query2, root_node, self.source_code);

        // parse `token_kind_to_terminal_syntax_kind`
        for m in query2_matches {
            let (_, token_kind, syntax_kind) =
                (m.captures[0].node, m.captures[1].node, m.captures[2].node);
            let better_node = syntax_kind.child_by_field_name("name").unwrap();
            let key_to_insert: String = self.str_from_node(better_node).into();
            let value_to_insert: String = self.str_from_node(token_kind).into();
            // println!("{key_to_insert} -> {value_to_insert}");
            self.kind_to_token.insert(key_to_insert, value_to_insert);
        }
        self
    }

    pub fn parse_take_token_identifier(mut self, root_node: Node) -> Self {
        // captures all `match` arms of the function `take_token_identifier`
        let query_token_identifier = "
        (function_item
            name: (identifier) @fn_name
            body: (block
                    (expression_statement
                      (match_expression
                          body: (match_block
                                    (match_arm pattern: (match_pattern) @str_pattern
                                               value: (scoped_identifier) @ident_value)))))
        (#eq? @fn_name \"take_token_identifier\"))";

        let query0 = Query::new(self.language_var, query_token_identifier).unwrap();
        let mut query_cursor0 = QueryCursor::new();
        let query0_matches = query_cursor0.matches(&query0, root_node, self.source_code);

        // parse `take_token_identifier`
        for m in query0_matches {
            let (_, str_pattern, ident_value) =
                (m.captures[0].node, m.captures[1].node, m.captures[2].node);
            let key_to_insert: String = self.str_from_node(ident_value).into();
            let mut value_to_insert: String = self.str_from_node(str_pattern).into();
            if value_to_insert == "_" {
                continue;
            } else {
                value_to_insert = value_to_insert.trim_matches('"').into();
                value_to_insert = format!("'{value_to_insert}'");
            }
            // println!("{key_to_insert} -> {value_to_insert}");
            let _ = self.token_to_str.insert(key_to_insert, value_to_insert);
        }
        self
    }

    pub fn match_terminal_call_expr(self, pattern: Node, function: Node, args: Node<'a>) -> Self {
        match self.str_from_node(function) {
            "take_token_literal_number" => self.take_token_literal_number(),
            "take_token_short_string" => self.take_token_short_string(),
            "take_token_string" => self.take_token_string(),
            "pick_kind" => self.pick_kind(pattern, args),
            "take_token_of_kind" => self.take_token_of_kind(pattern, args),
            "take_token_identifier" => self.take_token_identifier(),
            &_ => self,
        }
    }

    pub fn match_terminal_block(mut self, pattern: Node, block: Node<'a>) -> Self {
        let base_pattern = self.str_from_node(pattern).trim_matches('\'');
        let pattern_argument_query = "(match_expression
        body: (match_block
                (match_arm
                  pattern: (match_pattern) @inner_pattern
                  value: [
                    (scoped_identifier) @token_kind
                    (call_expression
                      function: (field_expression)
                      arguments: (arguments (scoped_identifier) @token_kind))
                  ])))";
        let query = Query::new(self.language_var, pattern_argument_query).unwrap();
        let mut query_cursor = QueryCursor::new();
        let query_matches = query_cursor.matches(&query, block, self.source_code);
        for m in query_matches {
            let (mut inner_pattern, token_kind) = (m.captures[0].node, m.captures[1].node);
            inner_pattern = inner_pattern.child(0).unwrap();
            println!("{}", inner_pattern.kind());
            let value: String;
            match inner_pattern.kind() {
                "tuple_struct_pattern" => {
                    let inner_char = self
                        .str_from_node(inner_pattern.child(2).unwrap())
                        .trim_matches('\'');
                    value = format!("'{base_pattern}{inner_char}'");
                }
                "_" => {
                    value = format!("'{base_pattern}'");
                }
                &_ => value = String::new(),
            }
            let key: String = self.str_from_node(token_kind).into();
            let _ = self.token_to_str.insert(key, value);
        }
        self
    }

    pub fn parse_match_terminal(self, root_node: Node<'a>) -> Self {
        // captures all `match` arms of the function `match_terminal`
        let query_match_terminal = "
        (function_item
            name: (identifier) @fn_name
            body: (block
                    (let_declaration
                      value:
                        (if_expression consequence: (block
                            (expression_statement
                              (match_expression
                                body: (match_block
                                        (match_arm) @token_match_arm)))))))
        (#eq? @fn_name \"match_terminal\"))";

        let query1 = Query::new(self.language_var, query_match_terminal).unwrap();
        let mut query_cursor1 = QueryCursor::new();
        let query1_matches = query_cursor1.matches(&query1, root_node, self.source_code);

        let mut new_self: LexerFileParser<'a> = self;
        // parse `match_terminal`
        for m in query1_matches {
            let (_fn_name, token_match_arm) = (m.captures[0].node, m.captures[1].node);
            let pattern = token_match_arm.child_by_field_name("pattern").unwrap();
            let value = token_match_arm.child_by_field_name("value").unwrap();
            match value.kind() {
                "call_expression" => {
                    let function = value
                        .child_by_field_name("function")
                        .unwrap()
                        .child_by_field_name("field")
                        .unwrap();
                    let args = value.child_by_field_name("arguments").unwrap();
                    new_self = new_self.match_terminal_call_expr(pattern, function, args);
                }
                "block" => {
                    new_self = new_self.match_terminal_block(pattern, value);
                }
                &_ => (),
            };
        }
        new_self
    }
}

pub fn parse_lexer(file: &str) -> (HashMap<String, String>, HashMap<String, String>) {
    // first initialize the tree-sitter objects
    let mut parser = Parser::new();

    let language_var = tree_sitter_rust::language();
    parser
        .set_language(language_var)
        .expect("Error loading Rust grammar");

    let source_code_string = fs::read_to_string(file).unwrap();
    let source_code_bytes = source_code_string.as_bytes();
    let tree = parser.parse(source_code_bytes, None).unwrap();
    let cursor = tree.walk();
    let root_node = tree.root_node();

    let mut lexer_file_parser = LexerFileParser::new(cursor, source_code_bytes, language_var);

    lexer_file_parser = lexer_file_parser.parse_take_token_identifier(root_node);
    lexer_file_parser = lexer_file_parser.parse_match_terminal(root_node);
    lexer_file_parser = lexer_file_parser.parse_token_kind_to_syntax_kind(root_node);

    // return the two hashmaps
    (
        lexer_file_parser.kind_to_token,
        lexer_file_parser.token_to_str,
    )
}
