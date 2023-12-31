use crate::parser_utils::*;
use crate::{ALLOW_EMPTY_LIST, BINARY_EXPR_NODE, LEADING_WHITESPACE, ROOT_NODE, UNARY_EXPR_NODE};
use std::{
    collections::{HashMap, HashSet},
    fs, str,
};
use tree_sitter::{Node, Parser, Query, QueryCursor, TreeCursor};

#[derive(Clone)]
struct CairoSpecParser<'a> {
    cursor: TreeCursor<'a>,
    language_var: tree_sitter::Language,
    source_code: &'a [u8],
    kind_to_token: HashMap<String, String>,
    token_to_str: HashMap<String, String>,
    option_to_str: HashMap<String, String>,
    list_to_str: HashMap<String, String>,
    unary_precedence: HashMap<u32, Vec<String>>,
    post_precedence: HashMap<u32, Vec<String>>,
    allow_empty_list: HashSet<String>,
    to_delete: HashSet<String>,
    source_code_rule: String,
}

impl<'a> CairoSpecParser<'a> {
    /// Creates a new parser.
    fn new(
        cursor: TreeCursor<'a>,
        language_var: tree_sitter::Language,
        source_code: &'a [u8],
        hashmaps: (HashMap<String, String>, HashMap<String, String>),
        precedences: (HashMap<u32, Vec<String>>, HashMap<u32, Vec<String>>),
        to_delete: HashSet<String>,
    ) -> Self {
        CairoSpecParser {
            cursor,
            language_var,
            source_code,
            kind_to_token: hashmaps.0,
            token_to_str: hashmaps.1,
            option_to_str: HashMap::<String, String>::new(),
            list_to_str: HashMap::<String, String>::new(),
            unary_precedence: precedences.0,
            post_precedence: precedences.1,
            allow_empty_list: HashSet::<String>::from_iter(
                ALLOW_EMPTY_LIST.iter().map(|s_ref| s_ref.to_string()),
            ),
            to_delete,
            source_code_rule: String::new(),
        }
    }
    /// Checks if `kind`:
    /// i) should be ignored if it belongs to `to_delete`, or
    /// ii) can be mapped via `kind_to_token` and` token_to_str`, or
    /// iii) can be mapped via `option_to_str`, or
    /// iv) can be mapped via `list_to_str`.
    /// Otherwise returns `default_str`.
    fn kind_to_str_or(&self, default_str: String, kind: String) -> String {
        if self.to_delete.contains(&kind) {
            String::new()
        } else {
            self.kind_to_token
                .get(&kind)
                .and_then(|token| self.token_to_str.get(token))
                .map(|s| s.to_string())
                .or(self.option_to_str.get(&kind).map(|s| s.to_string()))
                .or(self.list_to_str.get(&kind).map(|s| s.to_string()))
                .unwrap_or(default_str)
        }
    }

    fn kind_to_str(&self, kind: String) -> String {
        self.kind_to_str_or(kind.clone(), kind)
    }

    /// helper function to get the code represented by a given Node
    fn str_from_node(&self, n: Node<'a>) -> &'a str {
        std::str::from_utf8(&self.source_code[n.byte_range()]).unwrap()
    }

    fn format_struct_node(&self, args_vec: Vec<String>) -> String {
        let (field, node) = (&args_vec[0], &args_vec[1]);
        let default_str = format!(
            "field('{}', {})",
            camel_to_snake(field),
            self.kind_to_str_or(format!("$.{}", camel_to_snake(node)), node.to_string()),
        );
        if self.to_delete.contains(node) {
            String::new()
        } else {
            self.kind_to_token
                .get(node)
                .and_then(|s| self.token_to_str.get(s).map(|s_ref| s_ref.to_string()))
                .or(self
                    .option_to_str
                    .get(node)
                    .map(|s| format!("optional(field('{}', {s}))", camel_to_snake(field))))
                .unwrap_or(default_str)
                .clone()
        }
    }

    fn format_struct_node_optional_list(&self, args_vec: Vec<String>) -> String {
        let (field, node) = (&args_vec[0], &args_vec[1]);
        let default_str = format!(
            "field('{}', {})",
            camel_to_snake(field),
            self.kind_to_str_or(format!("$.{}", camel_to_snake(node)), node.to_string()),
        );
        if self.to_delete.contains(node) {
            String::new()
        } else {
            self.kind_to_token
                .get(node)
                .and_then(|s| self.token_to_str.get(s).map(|s_ref| s_ref.to_string()))
                .or(self
                    .option_to_str
                    .get(node)
                    .map(|s| format!("optional(field('{}', {s}))", camel_to_snake(field))))
                .or(self
                    .list_to_str
                    .get(node)
                    .map(|s| format!("optional(field('{}', {s}))", camel_to_snake(field))))
                .unwrap_or(default_str)
                .clone()
        }
    }

    /// Iterates over arguments of a call expression, returns a vector of String.
    fn iterate_arguments(&mut self, call_node: Node<'a>) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let mut cursor_clone = self.cursor.clone();
        for child in call_node.children(&mut cursor_clone) {
            match self.str_from_node(child) {
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
        let _ = self.option_to_str.insert(option_name, option_str);
    }

    fn preprocess_add_option(&mut self, n: Node<'a>) {
        let query_add_option = "
        (call_expression
            function: (field_expression field: (field_identifier) @method_name)
            arguments: (arguments) @node_args
            (#eq? @method_name \"add_option\"))";
        let query = Query::new(self.language_var, query_add_option).unwrap();
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
            function: (field_expression field: (field_identifier) @method_name)
            arguments: (arguments (call_expression
                function: (scoped_identifier)
                arguments: (arguments) @args))
            (#eq? @method_name \"add_struct\"))";
        let query = Query::new(self.language_var, query_empty_struct).unwrap();
        let mut query_cursor = QueryCursor::new();
        for m in query_cursor.matches(&query, n, self.source_code) {
            let captures = m.captures;
            let (_method_name, args) = (captures[0].node, captures[1].node);
            let struct_str = &self.iterate_arguments(args)[0];
            self.to_delete.insert(struct_str.into());
        }
    }

    fn process_add_expr_binary(&mut self, n: Node<'a>) -> String {
        let query_expr_binary = "
        (call_expression
            function: (field_expression value: (call_expression
                function: (field_expression value: (call_expression
                    function: (field_expression value: (call_expression
                        function: (scoped_identifier)
                        arguments: (arguments (string_literal) @struct_name)
                        (#eq? @struct_name \"\\\"ExprBinary\\\"\")))
                    arguments: (arguments) @lhs))
                arguments: (arguments) @op))
            arguments: (arguments) @rhs)";
        let query = Query::new(self.language_var, query_expr_binary).unwrap();
        let mut query_cursor = QueryCursor::new();
        let m = query_cursor
            .matches(&query, n, self.source_code)
            .next()
            .unwrap();
        let (lhs, _, rhs) = {
            let c = m.captures;
            (c[1].node, c[2].node, c[3].node)
        };
        let (lhs_field, lhs_kind) = {
            let args = self.iterate_arguments(lhs);
            (args[0].clone(), args[1].clone())
        };
        let (rhs_field, rhs_kind) = {
            let args = self.iterate_arguments(rhs);
            (args[0].clone(), args[1].clone())
        };
        let mut expr_binary: Vec<String> = Vec::new();
        expr_binary.push(format!("expr_binary: $ => choice("));
        for prec in self.post_precedence.keys() {
            let terminal_ops = self.post_precedence.get(prec).unwrap();
            expr_binary.push(format!("    prec.left({prec}, choice("));
            expr_binary.append(
                &mut terminal_ops
                    .into_iter()
                    .map(|t_op| {
                        self.kind_to_str(t_op.split("::").collect::<Vec<&str>>()[1].to_string())
                    })
                    .map(|s| {
                        format!(
                            "        seq(field('{lhs_field}', $.{}), {s}, field('{rhs_field}', $.{})),",
                            camel_to_snake(&lhs_kind),
                            camel_to_snake(&rhs_kind),
                        )
                    })
                    .collect::<Vec<String>>(),
            );
            expr_binary.push("    )),".to_string());
        }
        expr_binary.push("),".to_string());
        expr_binary.push("".to_string());
        join_lines(expr_binary)
    }

    fn process_add_expr_unary(&mut self, n: Node<'a>) -> String {
        let query_expr_unary = "
        (call_expression
            function: (field_expression value: (call_expression
                function: (field_expression value: (call_expression
                    function: (scoped_identifier)
                    arguments: (arguments (string_literal) @struct_name)
                    (#eq? @struct_name \"\\\"ExprUnary\\\"\")))
                arguments: (arguments) @op))
            arguments: (arguments) @expr)";
        let query = Query::new(self.language_var, query_expr_unary).unwrap();
        let mut query_cursor = QueryCursor::new();
        let m = query_cursor
            .matches(&query, n, self.source_code)
            .next()
            .unwrap();
        let rhs = m.captures[2].node;
        let (rhs_field, rhs_kind) = {
            let args = self.iterate_arguments(rhs);
            (args[0].clone(), args[1].clone())
        };
        let mut expr_unary: Vec<String> = Vec::new();
        expr_unary.push(format!("expr_unary: $ => choice("));
        for prec in self.unary_precedence.keys() {
            let terminal_ops = self.unary_precedence.get(prec).unwrap();
            expr_unary.push(format!("    prec.left({prec}, choice("));
            expr_unary.append(
                &mut terminal_ops
                    .into_iter()
                    .map(|t_op| {
                        self.kind_to_str(t_op.split("::").collect::<Vec<&str>>()[1].to_string())
                    })
                    .map(|s| {
                        format!(
                            "        seq({s}, field('{rhs_field}', $.{})),",
                            camel_to_snake(&rhs_kind),
                        )
                    })
                    .collect::<Vec<String>>(),
            );
            expr_unary.push("    )),".to_string());
        }
        expr_unary.push("),".to_string());
        expr_unary.push("".to_string());
        join_lines(expr_unary)
    }

    fn preprocess_add_list(&mut self, n: Node<'a>) {
        let query_add_list = "
        (call_expression
            function: (field_expression field: (field_identifier) @method_name)
            arguments: (arguments) @node_args
            (#eq? @method_name \"add_list\"))";
        let query = Query::new(self.language_var, query_add_list).unwrap();
        let mut query_cursor = QueryCursor::new();
        for m in query_cursor.matches(&query, n, self.source_code) {
            let args = self.iterate_arguments(m.captures[1].node);
            let list_element = camel_to_snake(&args[1]);
            self.list_to_str
                .insert(args[0].clone(), format!("repeat($.{list_element})"));
        }

        let query_add_separated_list = "
        (call_expression
            function: (field_expression field: (field_identifier) @method_name)
            arguments: (arguments) @node_args
            (#eq? @method_name \"add_separated_list\"))";
        let query = Query::new(self.language_var, query_add_separated_list).unwrap();
        let mut query_cursor = QueryCursor::new();
        for m in query_cursor.matches(&query, n, self.source_code) {
            let args = self.iterate_arguments(m.captures[1].node);
            let list_element = camel_to_snake(&args[1]);
            let sep = &args[2];
            let new_sep = self.kind_to_str(sep.clone());
            self.list_to_str.insert(
                args[0].clone(),
                format!("seq($.{list_element}, repeat(seq({new_sep}, $.{list_element})))"),
            );
        }
    }

    /// It is needed to preprocess the file in order to handle `add_option` calls and
    /// empty `add_struct` calls (like `add_struct(StructBuilder::new("ImplItemMissing"))`)
    fn preprocess_file(&mut self, n: Node<'a>) {
        self.preprocess_add_option(n.clone());
        self.preprocess_empty_struct(n.clone());
        self.preprocess_add_list(n);
    }

    /// create a tree-sitter `seq(...)` from an `add_struct` method call
    fn add_struct(&mut self, n: Node<'a>) -> String {
        let mut self_clone = self.clone();
        let args_node = _get_args_node(n).child(1).unwrap();
        let (inner_node, methods_args) = self.iterate_method_calls(args_node);
        let inner_args = self_clone.iterate_arguments(inner_node);
        if inner_args.is_empty()
            || methods_args.is_empty()
            || self.to_delete.contains(&inner_args[0])
        {
            String::new()
        } else {
            let mut result: Vec<String> = methods_args
                .into_iter()
                .map(|(_method, arg)| -> String {
                    let args_vec: Vec<String> = self_clone.iterate_arguments(arg);
                    if self.allow_empty_list.contains(inner_args[0].as_str()) {
                        self_clone.format_struct_node_optional_list(args_vec)
                    } else {
                        self_clone.format_struct_node(args_vec)
                    }
                })
                .filter(|s| !s.is_empty())
                .map(|s| format!("    {s},"))
                .collect::<Vec<String>>();
            result.push(String::from("),"));
            result.push(String::new());
            match inner_args[0].as_str() {
                BINARY_EXPR_NODE => self.process_add_expr_binary(args_node),
                UNARY_EXPR_NODE => self.process_add_expr_unary(args_node),
                ROOT_NODE => {
                    result.insert(0, String::from("source_code: $ => seq("));
                    self.source_code_rule = join_lines(result);
                    String::new()
                }
                &_ => {
                    result.insert(
                        0,
                        format!("{}: $ => seq(", camel_to_snake(inner_args[0].as_str())),
                    );
                    join_lines(result)
                }
            }
        }
    }

    /// create a tree-sitter choice from a `add_enum` method call
    fn add_enum(&mut self, n: Node<'a>) -> String {
        let mut self_clone = self.clone();
        let args_node = _get_args_node(n).child(1).unwrap();
        let (inner_node, methods_args) = self.iterate_method_calls(args_node);
        let enum_camel_case = self_clone.iterate_arguments(inner_node)[0].clone();
        if self.to_delete.contains(&enum_camel_case) {
            String::new()
        } else {
            let enum_name = camel_to_snake(&enum_camel_case);
            let result: Vec<String> = methods_args
                .into_iter()
                .map(|(method, arg)| -> String {
                    let method_name = method.child_by_field_name("field").unwrap();
                    let args: Vec<String> = self_clone.iterate_arguments(arg);
                    match self.str_from_node(method_name) {
                        "node" | "missing" => self_clone.kind_to_str_or(
                            format!("$.{enum_name}_{}", camel_to_snake(&args[0])),
                            enum_camel_case.clone() + &args[0],
                        ),
                        "node_with_explicit_kind" => self_clone.kind_to_str_or(
                            format!("$.{}", camel_to_snake(&args[1])),
                            args[1].clone(),
                        ),
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
    precedences: (HashMap<u32, Vec<String>>, HashMap<u32, Vec<String>>),
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

    let mut cairo_parser = CairoSpecParser::new(
        cursor,
        language_var,
        source_code_bytes,
        hashmaps,
        precedences,
        to_delete,
    );
    // first step: parse all `add_option` method calls, ignore empty `add_struct`
    cairo_parser.preprocess_file(root_call_expr.clone());
    // second step: generate `grammar.js`
    cairo_parser.iterate_nodes_aggregator(root_call_expr)
}
