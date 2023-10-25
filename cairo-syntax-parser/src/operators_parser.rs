use crate::parser_utils::*;
use std::{collections::HashMap, fs, str};
use tree_sitter::{Node, Parser, Query, QueryCursor, TreeCursor};

struct OperatorsParser<'a> {
    cursor: TreeCursor<'a>,
    source_code: &'a [u8],
    unary_precedence: HashMap<String, u32>,
    post_precedence: HashMap<String, u32>,
}
impl<'a> OperatorsParser<'a> {
    fn new(cursor: TreeCursor<'a>, source_code: &'a [u8]) -> Self {
        // create hashmaps
        let unary_precedence: HashMap<String, u32> = HashMap::new();
        let post_precedence: HashMap<String, u32> = HashMap::new();
        OperatorsParser {
            cursor,
            source_code,
            unary_precedence,
            post_precedence,
        }
    }
    fn parse_get_unary_operator_precedence(&mut self) {
        todo!();
    }

    fn parse_get_post_operator_precedence(&mut self) {
        todo!();
    }
}

fn build_query(fn_name: &str) -> String {
    format!(
        "
    (function_item
        name: (identifier) @fn_name
        body: (block
                (expression_statement
                  (match_expression
                      value: (identifier)
                      body: (match_block
                              (match_arm
                                  pattern: (match_pattern) @pattern
                                  value: [
                                    (call_expression)
                                    (identifier)
                                  ] @match_value)))))
        (#eq? @fn_name \"{fn_name}\"))"
    )
}

pub fn parse_operators(file: &str) -> (HashMap<String, u32>, HashMap<String, u32>) {
    let mut parser = Parser::new();
    let language_var = tree_sitter_rust::language();
    parser
        .set_language(language_var)
        .expect("Error loading Rust grammar");

    let source_code_string = fs::read_to_string(file).unwrap();
    let source_code = source_code_string.as_bytes();

    let tree = parser.parse(source_code, None).unwrap();
    let cursor = tree.walk();
    let root_node = tree.root_node();

    let mut operators_parser = OperatorsParser::new(cursor, source_code);

    let query_unary_precedence = &build_query("get_unary_operator_precedence");
    let ts_query = Query::new(language_var, query_unary_precedence).unwrap();
    let mut query_cursor = QueryCursor::new();
    for query_matches in query_cursor
        .matches(&ts_query, root_node, source_code)
        .next()
        .unwrap()
        .captures
    {}

    operators_parser.parse_get_unary_operator_precedence();
    operators_parser.parse_get_post_operator_precedence();
    (
        operators_parser.unary_precedence,
        operators_parser.post_precedence,
    )
}
