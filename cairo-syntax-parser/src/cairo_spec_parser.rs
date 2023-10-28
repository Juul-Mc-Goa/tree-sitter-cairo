use crate::parser_utils::*;
use crate::{LEADING_WHITESPACE, ROOT_NODE};
use std::{
    collections::{HashMap, HashSet},
    fs, str,
};
use tree_sitter::{Node, Parser, Query, QueryCursor, TreeCursor};

#[derive(Clone)]
struct CairoSpecParser<'a> {
    cursor: TreeCursor<'a>,
    source_code: &'a [u8],
    kind_to_token: HashMap<String, String>,
    token_to_str: HashMap<String, String>,
    option_to_str: HashMap<String, String>,
    to_delete: HashSet<String>,
    source_code_rule: String,
}

impl<'a> CairoSpecParser<'a> {
    /// Creates a new parser.
    fn new(
        cursor: TreeCursor<'a>,
        source_code: &'a [u8],
        hashmaps: (HashMap<String, String>, HashMap<String, String>),
        to_delete: HashSet<String>,
    ) -> Self {
        CairoSpecParser {
            cursor,
            source_code,
            kind_to_token: hashmaps.0,
            token_to_str: hashmaps.1,
            option_to_str: HashMap::<String, String>::new(),
            to_delete,
            source_code_rule: String::new(),
        }
    }
    /// Checks if `kind`:
    /// i) can be mapped via `kind_to_token, token_to_str`, or
    /// ii) can be mapped via `option_to_str`, or
    /// iii) is replaced by an empty string if it belongs to `to_delete`.
    /// Otherwise returns `default_str`.
    fn kind_to_str_or(&self, default_str: String, kind: String) -> String {
        match self
            .kind_to_token
            .get(&kind)
            .map(|token| self.token_to_str.get(token))
            .flatten()
        {
            Some(string_ref) => string_ref.into(),
            None => match self
                .option_to_str
                .get(&kind)
                .map(|option_str| String::from(option_str))
            {
                Some(other_str_ref) => other_str_ref.into(),
                None => {
                    if self.to_delete.contains(&kind) {
                        String::new()
                    } else {
                        default_str
                    }
                }
            },
        }
    }

    /// helper function to get the code represented by a given Node
    fn str_from_node(&self, n: Node<'a>) -> &'a str {
        std::str::from_utf8(&self.source_code[n.byte_range()]).unwrap()
    }

    /// iterate over arguments of a call expression
    fn iterate_arguments(&mut self, call_node: Node<'a>) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for child in call_node.children(&mut self.cursor) {
            match str_from_node(child, self.source_code) {
                "," => (),
                "(" => (),
                ")" => (),
                arg => result.push(String::from(arg.trim_matches('"'))),
            }
        }
        result
    }

    /// Takes a tree of the form `call_expression -> function -> field -> call_expression -> function -> ...`
    /// and outputs a vector of tuples of nodes `(function, arguments)`.
    fn iterate_method_calls(&mut self, n: Node<'a>) -> (Node<'a>, Vec<(Node<'a>, Node<'a>)>) {
        self.cursor.reset(n);
        let mut has_children = true;
        let mut result: Vec<(Node, Node)> = Vec::new();
        let mut leaf: Node = n;
        while has_children {
            let node = self.cursor.node();
            let node_args = node.child_by_field_name("arguments").unwrap();
            let node_method = node.child_by_field_name("function").unwrap();
            match node_method.child_by_field_name("field") {
                Some(_) => {
                    result.push((node_method, node_args));
                    self.cursor.goto_first_child();
                    self.cursor.goto_first_child();
                    has_children = self
                        .cursor
                        .node()
                        .child_by_field_name("arguments")
                        .is_some();
                }
                None => {
                    leaf = node_args;
                    break;
                }
            }
        }
        // revert result to obtain the correct ordering of method calls
        (leaf, result.into_iter().rev().collect())
    }

    fn add_option_to_hashmap(&mut self, camel_node_str: String) {
        let snake_node_str = camel_to_snake(&camel_node_str);
        let option_name = format!("Option{}", camel_node_str);
        let option_str = self.kind_to_str_or(
            format!("$.{}", snake_node_str),
            String::from(camel_node_str),
        );
        let _ = self
            .option_to_str
            .insert(option_name, format!("optional({option_str})"));
    }

    fn preprocess_add_option(&mut self, n: Node<'a>) {
        let query_add_option = "
        (call_expression
            :function (field_expression
                :field (field_identifier) @method_name
                :arguments (arguments) @node_args)
            (#eq? method_name \"add_option\"))";
        let query = Query::new(language_var, query_add_option).unwrap();
        let mut query_cursor = QueryCursor::new();
        for m in query_cursor.matches(&query, n, self.source_code) {
            let node_args = m.captures[1].node;
            let option_to_add = &self.clone().iterate_arguments(node_args)[0];
            self.add_option_to_hashmap(String::from(option_to_add));
        }
    }

    fn preprocess_empty_struct(&mut self, n: Node<'a>) {
        let query_empty_struct = "
        (call_expression
            :function (field_expression :field (field_identifier) @method_name)
            :arguments (call_expression
                :function (scoped_identifier)
                :arguments (arguments) @args)
            (#eq? method_name \"add_struct\"))";
        let query = Query::new(language_var, query_empty_struct).unwrap();
        let mut query_cursor = QueryCursor::new();
        for m in query_cursor.matches(&query, n, self.source_code) {
            let captures = m.captures;
            let (_method_name, args) = (captures[0].node, captures[1].node);
            let struct_str = self.iterate_arguments(args)[0];
            self.to_delete.insert(struct_str.into());
        }
    }

    fn preprocess_add_expr_binary(&mut self, n: Node<'a>) {
        let query_expr_binary = "
        (call_expression
            :function (field_expression :field (field_identifier) @method_name)
            :arguments (call_expression
                :function (field_expression :value (call_expression
                    :function (field_expression :value (call_expression
                        :function (field_expression :value (call_expression
                            :function (scoped_identifier)
                            :arguments (arguments (string_literal) @struct_name)
                            (#eq? @struct_name.1 \"ExprBinary\")))
                        :arguments (arguments) @lhs))
                    :arguments (arguments) @op))
                :arguments (arguments) @rhs))
            (#eq? method_name \"add_struct\"))";
        let query = Query::new(language_var, query_expr_binary).unwrap();
        let mut query_cursor = QueryCursor::new();
        for m in query_cursor.matches(&query, n, self.source_code) {
            let (lhs, op, rhs) = {
                let c = m.captures;
                (c[2].node, c[3].node, c[4].node)
            };
        }
    }
    fn preprocess_add_expr_unary(&mut self, n: Node<'a>) {
        let query_expr_unary = "
        (call_expression
            :function (field_expression :field (field_identifier) @method_name)
            :arguments (call_expression
                :function (field_expression :value (call_expression
                    :function (field_expression :value (call_expression
                        :function (scoped_identifier)
                        :arguments (arguments (string_literal) @struct_name)
                        (#eq? @struct_name.1 \"ExprUnary\")))
                    :arguments (arguments) @op))
                :arguments (arguments) @expr))
            (#eq? method_name \"add_struct\"))";
        let query = Query::new(language_var, query_expr_unary).unwrap();
        let mut query_cursor = QueryCursor::new();
        let mut query_matches = query_cursor.matches(&query, n, self.source_code);
    }

    /// It is needed to preprocess the file in order to handle `add_option` calls and
    /// empty `add_struct` calls (like `add_struct(StructBuilder::new("ImplItemMissing"))`)
    fn preprocess_file(&mut self, n: Node<'a>) {
        self.cursor.reset(n);
        let mut has_children = true;
        let query = Query::new(language_var, query_root_node).unwrap();
        let mut query_cursor = QueryCursor::new();
        let mut query_matches = query_cursor.matches(&query, root_node, self.source_code);

        let captures = query_matches.next().unwrap().captures;
        while has_children {
            let node = self.cursor.node();
            let node_args = node.child_by_field_name("arguments").unwrap();
            let node_method = node.child_by_field_name("function").unwrap();
            match node_method.child_by_field_name("field") {
                Some(field) => {
                    match self.str_from_node(field) {
                        "add_option" => {
                            let option_to_add = &self.clone().iterate_arguments(node_args)[0];
                            self.add_option_to_hashmap(String::from(option_to_add));
                        }
                        "add_struct" => {
                            let kind_arguments = node_args
                                .child(1)
                                .map(|first_arg| {
                                    (
                                        first_arg.child_by_field_name("function"),
                                        first_arg.child_by_field_name("arguments"),
                                    )
                                })
                                .map(|(f, a)| (f.map(|f1| f1.kind()), a));
                            match kind_arguments {
                                Some((Some("scoped_identifier"), Some(arguments))) => {
                                    let struct_str = self.iterate_arguments(arguments)[0].clone();
                                    // iterate_arguments modifies the cursor so we need to reset the cursor
                                    self.cursor.reset(node);
                                    self.to_delete.insert(struct_str.into());
                                }
                                _ => (),
                            }
                        }
                        &_ => (),
                    }
                    self.cursor.goto_first_child();
                    self.cursor.goto_first_child();
                    has_children = self
                        .cursor
                        .node()
                        .child_by_field_name("arguments")
                        .is_some();
                }
                None => {
                    break;
                }
            }
        }
    }

    /// create a tree-sitter `seq(...)` from an `add_struct` method call
    fn add_struct(&mut self, n: Node<'a>) -> String {
        let mut self_clone = self.clone();
        let args_node = _get_args_node(n).child(1).unwrap();
        let (inner_node, methods_args) = self.iterate_method_calls(args_node);
        let result: Vec<String> = methods_args
            .into_iter()
            .map(|(_method, arg)| -> String {
                let args_vec: Vec<String> = self_clone.iterate_arguments(arg);
                let field_name = camel_to_snake(&args_vec[0]);
                let field_value = camel_to_snake(&args_vec[1]);
                self_clone.kind_to_str_or(
                    format!("field('{field_name}', $.{field_value})"),
                    args_vec[1].clone(),
                )
            })
            .collect::<Vec<String>>();

        if result.len() == 0 {
            String::new()
        } else {
            let inner_args = self_clone.iterate_arguments(inner_node);
            let struct_name = if inner_args.len() == 0 {
                String::from("")
            } else {
                camel_to_snake(&inner_args[0])
            };
            let mut end_result = result
                .into_iter()
                .filter(|s| !s.is_empty())
                .map(|s| format!("    {s},"))
                .collect::<Vec<String>>();
            end_result.push(String::from("),"));
            end_result.push(String::new());
            if &inner_args[0] == ROOT_NODE {
                end_result.insert(0, String::from("source_code: $ => seq("));
                self.source_code_rule = join_lines(end_result);
                String::new()
            } else {
                end_result.insert(0, format!("{struct_name}: $ => seq("));
                join_lines(end_result)
            }
        }
    }

    /// create a tree-sitter choice from a `add_enum` method call
    fn add_enum(&mut self, n: Node<'a>) -> String {
        let mut self_clone = self.clone();
        let args_node = _get_args_node(n).child(1).unwrap();
        let (inner_node, methods_args) = self.iterate_method_calls(args_node);
        let enum_camel_case = self_clone.iterate_arguments(inner_node)[0].clone();
        let enum_name = camel_to_snake(&enum_camel_case);
        let result: Vec<String> = methods_args
            .into_iter()
            .map(|(method, arg)| -> String {
                let method_name = method.child_by_field_name("field").unwrap();
                let args: Vec<String> = self_clone.iterate_arguments(arg);
                match str_from_node(method_name, self_clone.source_code) {
                    "node" | "missing" => self_clone.kind_to_str_or(
                        format!("$.{enum_name}_{}", camel_to_snake(&args[0])),
                        enum_camel_case.clone() + &args[0],
                    ),
                    "node_with_explicit_kind" => self_clone
                        .kind_to_str_or(format!("$.{}", camel_to_snake(&args[1])), args[1].clone()),
                    &_ => String::from(""),
                }
            })
            .collect::<Vec<String>>();
        let mut end_result = result
            .into_iter()
            .filter(|s| !s.is_empty())
            .map(|s| format!("    {s},"))
            .collect::<Vec<String>>();
        end_result.insert(0, format!("{enum_name}: $ => choice("));
        end_result.push(String::from("),"));
        end_result.push(String::new());
        join_lines(end_result)
    }

    /// Rule for matching list of elements. `repeat1` is used to ensure the rule
    /// won't match empty strings (which would result in an error).
    fn add_list(&mut self, n: Node<'a>) -> String {
        let args_node = _get_args_node(n);
        let args = self.iterate_arguments(args_node);
        let list_name = camel_to_snake(&args[0]);
        let list_element = camel_to_snake(&args[1]);
        format!("{LEADING_WHITESPACE}{list_name}: $ => repeat1($.{list_element}),\n")
    }

    fn add_separated_list(&mut self, n: Node) -> String {
        let args_node = _get_args_node(n);
        let args = self.clone().iterate_arguments(args_node);
        let list_name = camel_to_snake(&args[0]);
        let list_element = camel_to_snake(&args[1]);
        let sep = &args[2];
        let new_sep = self.kind_to_str_or(sep.clone(), sep.into());
        format!(
            "{LEADING_WHITESPACE}{list_name}: $ => seq($.{list_element}, repeat(seq({new_sep}, $.{list_element}))),\n"
        )
    }

    fn add_option(&mut self, _n: Node) -> String {
        String::new()
    }

    fn add_token_terminal(&mut self, _n: Node) -> String {
        String::new()
    }

    fn add_kw_token_terminal(&mut self, _n: Node) -> String {
        String::new()
    }

    fn add_token(&mut self, _n: Node) -> String {
        String::new()
    }

    fn iterate_nodes_aggregator(&mut self, nodes_aggregator: Node<'a>) -> String {
        let mut self_clone = self.clone();
        let (_, methods_args) = self.iterate_method_calls(nodes_aggregator);
        let result: Vec<String> = methods_args
            .into_iter()
            .map(|(method, _arg)| -> String {
                let method_name = method.child_by_field_name("field").unwrap();
                match str_from_node(method_name, self_clone.source_code) {
                    "add_list" => self_clone.add_list(method_name),
                    "add_separated_list" => self_clone.add_separated_list(method_name),
                    "add_enum" => self_clone.add_enum(method_name),
                    "add_struct" => self_clone.add_struct(method_name),
                    "add_option" => self_clone.add_option(method_name),
                    "add_token_and_terminal" => self_clone.add_token_terminal(method_name),
                    "add_keyword_token_and_terminal" => {
                        self_clone.add_kw_token_terminal(method_name)
                    }
                    "add_token" => self_clone.add_token(method_name),
                    &_ => String::from("unknown"),
                }
            })
            .collect::<Vec<String>>();
        let mut new_result = self_clone.source_code_rule;
        new_result.push_str("\n");
        new_result.push_str(
            &self
                .token_to_str
                .get("")
                .expect("no key \"\" in token_to_str")
                .clone(),
        );
        new_result.push_str(
            &result
                .into_iter()
                .filter(|s| !s.is_empty())
                .collect::<Vec<String>>()
                .join("\n"),
        );
        new_result.into()
    }
}

fn _get_args_node(n: Node) -> Node {
    n.parent()
        .unwrap()
        .parent()
        .unwrap()
        .child_by_field_name("arguments")
        .unwrap()
}

/// main function, parses `cairo_spec.rs`
pub fn parse_cairo_spec(
    file: &str,
    hashmaps: (HashMap<String, String>, HashMap<String, String>),
    to_delete: HashSet<String>,
) -> String {
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

    // then create parser, and start parsing
    let query_root_node = "(call_expression
        function: (field_expression
                    value: (call_expression) @root_call
                    field: (field_identifier) @field_name
                    (#eq? @field_name \"get\")))";

    let query = Query::new(language_var, query_root_node).unwrap();
    let mut query_cursor = QueryCursor::new();
    let mut query_matches = query_cursor.matches(&query, root_node, source_code_bytes);

    let captures = query_matches.next().unwrap().captures;
    let root_call_expr = captures[0].node;

    let mut cairo_parser = CairoSpecParser::new(cursor, source_code_bytes, hashmaps, to_delete);
    // first step: parse all `add_option` method calls, ignore empty `add_struct`
    cairo_parser.preprocess_file(root_call_expr.clone());
    // second step: generate `grammar.js`
    cairo_parser.iterate_nodes_aggregator(root_call_expr)
}
