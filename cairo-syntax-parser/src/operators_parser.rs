use std::{collections::HashMap, fs, str};
use tree_sitter::{Node, Parser, Query, QueryCursor};

#[derive(Clone)]
struct OperatorsParser<'a> {
    source_code: &'a [u8],
    language_var: tree_sitter::Language,
    unary_precedence: HashMap<u32, Vec<String>>,
    post_precedence: HashMap<u32, Vec<String>>,
}
impl<'a> OperatorsParser<'a> {
    fn new(source_code: &'a [u8], language_var: tree_sitter::Language) -> Self {
        // create hashmaps
        let unary_precedence: HashMap<u32, Vec<String>> = HashMap::new();
        let post_precedence: HashMap<u32, Vec<String>> = HashMap::new();
        OperatorsParser {
            source_code,
            language_var,
            unary_precedence,
            post_precedence,
        }
    }
    /// helper function to get the code represented by a given Node
    fn str_from_node(&self, n: Node<'a>) -> &'a str {
        std::str::from_utf8(&self.source_code[n.byte_range()]).unwrap()
    }

    fn iterate_or_nodes(&self, n: Node<'a>) -> Vec<Node> {
        match n.kind() {
            "or_pattern" => {
                let query_or = "(or_pattern (scoped_identifier) @syntax_kind)";
                let ts_query = Query::new(self.language_var, query_or).unwrap();
                let mut query_cursor = QueryCursor::new();
                query_cursor
                    .matches(&ts_query, n, self.source_code)
                    .map(|m| m.captures[0].node)
                    .collect::<Vec<Node>>()
            }
            &_ => {
                let mut result: Vec<Node> = Vec::new();
                result.push(n);
                result
            }
        }
    }

    fn parse_operator_precedence(&mut self, root_node: Node, kind: &str) {
        let query_op_precedence = &build_query(&format!("get_{kind}_operator_precedence"));
        let ts_query = Query::new(self.language_var, query_op_precedence).unwrap();
        let mut query_cursor = QueryCursor::new();
        for query_match in query_cursor.matches(&ts_query, root_node, self.source_code) {
            let captures = query_match.captures;
            let (_fn_name, pattern, match_value) =
                (captures[0].node, captures[1].node, captures[2].node);
            let self_clone = self.clone();
            let or_nodes = self_clone
                .iterate_or_nodes(pattern)
                .into_iter()
                .map(|n| String::from(self.str_from_node(n)))
                .collect::<Vec<String>>();
            let precedence: u32 = self
                .str_from_node(
                    match_value
                        .child_by_field_name("arguments")
                        .unwrap()
                        .child(1)
                        .unwrap(),
                )
                .parse()
                .unwrap();
            if kind == "unary" {
                self.unary_precedence.insert(precedence, or_nodes);
            } else {
                self.post_precedence.insert(precedence, or_nodes);
            }
        }
    }

    fn parse_get_unary_operator_precedence(&mut self, root_node: Node) {
        self.parse_operator_precedence(root_node, "unary");
    }

    fn parse_get_post_operator_precedence(&mut self, root_node: Node) {
        self.parse_operator_precedence(root_node, "post");
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
                                  pattern: (match_pattern
                                              [
                                                (or_pattern (scoped_identifier))
                                                (scoped_identifier)
                                              ] @pattern)
                                  value: (call_expression) @match_value)))))
        (#eq? @fn_name \"{fn_name}\"))"
    )
}

pub fn parse_operators(file: &str) -> (HashMap<u32, Vec<String>>, HashMap<u32, Vec<String>>) {
    let mut parser = Parser::new();
    let language_var = tree_sitter_rust::language();
    parser
        .set_language(language_var)
        .expect("Error loading Rust grammar");

    let source_code_string = fs::read_to_string(file).unwrap();
    let source_code = source_code_string.as_bytes();

    let tree = parser.parse(source_code, None).unwrap();
    let root_node = tree.root_node();

    let mut operators_parser = OperatorsParser::new(source_code, language_var);

    operators_parser.parse_get_unary_operator_precedence(root_node);
    operators_parser.parse_get_post_operator_precedence(root_node);
    (
        operators_parser.unary_precedence,
        operators_parser.post_precedence,
    )
}
