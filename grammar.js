module.exports = grammar({
    name: 'cairo',

    rules: {
        source_code: $ => seq(
            field('items', $.item_list),
        ),
        
        type_suffix: _ => /_[0-9a-zA-Z]+/,

        escape_sequence: _ => /\\./,

        token_single_line_comment: _ => /\/\/.*/,

        token_newline: _ => /\n/,

        token_whitespace: _ => /[ \r\t]/,
        
        terminal_literal_number: $ => token(seq(
            choice(
              /[0-9][0-9_]*/,
              /0x[0-9a-fA-F_]+/,
              /0b[01_]+/,
              /0o[0-7_]+/,
            ),
            optional(/_[0-9a-zA-Z]+/),
        )),
                
        terminal_short_string: $ => seq(
            '\'',
            repeat(choice($.escape_sequence, /[^']/)),
            '\'',
            optional($.type_suffix),
        ),
                
        terminal_string: $ => seq(
            '"',
            repeat(choice($.escape_sequence, /[^"]/)),
            '"',
            optional($.type_suffix),
        ),
        
        terminal_identifier: _ => /[a-zA-Z_][a-zA-Z0-9_]*/,
        trivia: $ => repeat1($.trivium),

        trivium: $ => choice(
            $.token_single_line_comment,
            $.token_whitespace,
            $.token_newline,
        ),
        
        expr: $ => choice(
            $.expr_path,
            $.terminal_literal_number,
            $.terminal_short_string,
            $.terminal_string,
            'false',
            'true',
            $.expr_parenthesized,
            $.expr_unary,
            $.expr_binary,
            $.expr_list_parenthesized,
            $.expr_function_call,
            $.expr_struct_ctor_call,
            $.expr_block,
            $.expr_match,
            $.expr_if,
            $.expr_loop,
            $.expr_error_propagate,
            $.expr_field_init_shorthand,
            $.expr_indexed,
            $.expr_inline_macro,
        ),
        
        expr_list: $ => seq($.expr, repeat(seq(',', $.expr))),

        arg: $ => seq(
            field('modifiers', $.modifier_list),
            field('arg_clause', $.arg_clause),
        ),
        
        arg_clause: $ => choice(
            $.arg_clause_unnamed,
            $.arg_clause_named,
            $.arg_clause_field_init_shorthand,
        ),
        
        arg_clause_named: $ => seq(
            field('name', $.terminal_identifier),
            ':',
            field('value', $.expr),
        ),
        
        arg_clause_unnamed: $ => seq(
            field('value', $.expr),
        ),
        
        arg_clause_field_init_shorthand: $ => seq(
            ':',
            field('name', $.expr_field_init_shorthand),
        ),
        
        expr_field_init_shorthand: $ => seq(
            field('name', $.terminal_identifier),
        ),
        
        arg_list: $ => seq($.arg, repeat(seq(',', $.arg))),

        path_segment: $ => choice(
            $.path_segment_simple,
            $.path_segment_with_generic_args,
        ),
        
        path_segment_simple: $ => seq(
            field('ident', $.terminal_identifier),
        ),
        
        path_segment_with_generic_args: $ => seq(
            field('ident', $.terminal_identifier),
            optional('::'),
            field('generic_args', $.generic_args),
        ),
        
        expr_path: $ => seq($.path_segment, repeat(seq('::', $.path_segment))),

        expr_parenthesized: $ => seq(
            '(',
            field('expr', $.expr),
            ')',
        ),
        
        expr_unary: $ => seq(
            field('op', $.unary_operator),
            field('expr', $.expr),
        ),
        
        unary_operator: $ => choice(
            '!',
            '~',
            '-',
            '@',
            '*',
        ),
        
        expr_binary: $ => seq(
            field('lhs', $.expr),
            field('op', $.binary_operator),
            field('rhs', $.expr),
        ),
        
        binary_operator: $ => choice(
            '.',
            '!',
            '*',
            '*=',
            '/',
            '/=',
            '%',
            '%=',
            '+',
            '+=',
            '-',
            '-=',
            '==',
            '!=',
            '=',
            '&',
            '&&',
            '|',
            '||',
            '^',
            '<=',
            '>=',
            '<',
            '>',
        ),
        
        expr_list_parenthesized: $ => seq(
            '(',
            field('expressions', $.expr_list),
            ')',
        ),
        
        expr_function_call: $ => seq(
            field('path', $.expr_path),
            field('arguments', $.arg_list_parenthesized),
        ),
        
        arg_list_parenthesized: $ => seq(
            '(',
            field('args', $.arg_list),
            ')',
        ),
        
        expr_struct_ctor_call: $ => seq(
            field('path', $.expr_path),
            field('arguments', $.struct_arg_list_braced),
        ),
        
        struct_arg_list_braced: $ => seq(
            '{',
            field('arguments', $.struct_arg_list),
            '}',
        ),
        
        expr_block: $ => seq(
            '{',
            field('statements', $.statement_list),
            '}',
        ),
        
        expr_match: $ => seq(
            'match',
            field('expr', $.expr),
            '{',
            field('arms', $.match_arms),
            '}',
        ),
        
        match_arms: $ => seq($.match_arm, repeat(seq(',', $.match_arm))),

        match_arm: $ => seq(
            field('pattern', $.pattern),
            '=>',
            field('expression', $.expr),
        ),
        
        expr_if: $ => seq(
            'if',
            field('condition', $.expr),
            field('if_block', $.expr_block),
            optional($.else_clause),
        ),
        
        block_or_if: $ => choice(
            $.expr_block,
            $.expr_if,
        ),
        
        expr_loop: $ => seq(
            'loop',
            field('body', $.expr_block),
        ),
        
        else_clause: $ => seq(
            'else',
            field('else_block_or_if', $.block_or_if),
        ),
        
        expr_error_propagate: $ => seq(
            field('expr', $.expr),
            '?',
        ),
        
        expr_indexed: $ => seq(
            field('expr', $.expr),
            '[',
            field('index_expr', $.expr),
            ']',
        ),
        
        expr_inline_macro: $ => seq(
            field('path', $.expr_path),
            '!',
            field('arguments', $.wrapped_arg_list),
        ),
        
        struct_arg_expr: $ => seq(
            ':',
            field('expr', $.expr),
        ),
        
        struct_arg_single: $ => seq(
            field('identifier', $.terminal_identifier),
            optional($.struct_arg_expr),
        ),
        
        struct_arg_tail: $ => seq(
            '..',
            field('expression', $.expr),
        ),
        
        struct_arg: $ => choice(
            $.struct_arg_single,
            $.struct_arg_tail,
        ),
        
        struct_arg_list: $ => seq($.struct_arg, repeat(seq(',', $.struct_arg))),

        arg_list_braced: $ => seq(
            '{',
            field('arguments', $.arg_list),
            '}',
        ),
        
        arg_list_bracketed: $ => seq(
            '[',
            field('arguments', $.arg_list),
            ']',
        ),
        
        wrapped_arg_list: $ => choice(
            $.arg_list_bracketed,
            $.arg_list_parenthesized,
            $.arg_list_braced,
        ),
        
        pattern: $ => choice(
            '_',
            $.terminal_literal_number,
            $.terminal_short_string,
            $.terminal_string,
            $.pattern_identifier,
            $.pattern_struct,
            $.pattern_tuple,
            $.pattern_enum,
            $.expr_path,
        ),
        
        pattern_identifier: $ => seq(
            field('modifiers', $.modifier_list),
            field('name', $.terminal_identifier),
        ),
        
        pattern_struct: $ => seq(
            field('path', $.expr_path),
            '{',
            field('params', $.pattern_struct_param_list),
            '}',
        ),
        
        pattern_struct_param_list: $ => seq($.pattern_struct_param, repeat(seq(',', $.pattern_struct_param))),

        pattern_tuple: $ => seq(
            '(',
            field('patterns', $.pattern_list),
            ')',
        ),
        
        pattern_list: $ => seq($.pattern, repeat(seq(',', $.pattern))),

        pattern_struct_param: $ => choice(
            $.pattern_identifier,
            $.pattern_struct_param_with_expr,
            '..',
        ),
        
        pattern_struct_param_with_expr: $ => seq(
            field('modifiers', $.modifier_list),
            field('name', $.terminal_identifier),
            ':',
            field('pattern', $.pattern),
        ),
        
        pattern_enum: $ => seq(
            field('path', $.expr_path),
            optional($.pattern_enum_inner_pattern),
        ),
        
        pattern_enum_inner_pattern: $ => seq(
            '(',
            field('pattern', $.pattern),
            ')',
        ),
        
        type_clause: $ => seq(
            ':',
            field('ty', $.expr),
        ),
        
        return_type_clause: $ => seq(
            '->',
            field('ty', $.expr),
        ),
        
        statement: $ => choice(
            $.statement_let,
            $.statement_expr,
            $.statement_continue,
            $.statement_return,
            $.statement_break,
        ),
        
        statement_list: $ => repeat1($.statement),

        statement_let: $ => seq(
            'let',
            field('pattern', $.pattern),
            optional($.type_clause),
            '=',
            field('rhs', $.expr),
            ';',
        ),
        
        statement_expr: $ => seq(
            field('expr', $.expr),
            optional(';'),
        ),
        
        statement_continue: $ => seq(
            'continue',
            ';',
        ),
        
        expr_clause: $ => seq(
            field('expr', $.expr),
        ),
        
        statement_return: $ => seq(
            'return',
            optional($.expr_clause),
            ';',
        ),
        
        statement_break: $ => seq(
            'break',
            optional($.expr_clause),
            ';',
        ),
        
        param: $ => seq(
            field('modifiers', $.modifier_list),
            field('name', $.terminal_identifier),
            field('type_clause', $.type_clause),
        ),
        
        modifier_list: $ => repeat1($.modifier),

        modifier: $ => choice(
            'ref',
            'mut',
        ),
        
        param_list: $ => seq($.param, repeat(seq(',', $.param))),

        implicits_clause: $ => seq(
            'implicits',
            '(',
            field('implicits', $.implicits_list),
            ')',
        ),
        
        implicits_list: $ => seq($.expr_path, repeat(seq(',', $.expr_path))),

        function_signature: $ => seq(
            '(',
            field('parameters', $.param_list),
            ')',
            optional($.return_type_clause),
            optional($.implicits_clause),
            optional('nopanic'),
        ),
        
        member: $ => seq(
            field('attributes', $.attribute_list),
            field('name', $.terminal_identifier),
            field('type_clause', $.type_clause),
        ),
        
        member_list: $ => seq($.member, repeat(seq(',', $.member))),

        variant: $ => seq(
            field('attributes', $.attribute_list),
            field('name', $.terminal_identifier),
            optional($.type_clause),
        ),
        
        variant_list: $ => seq($.variant, repeat(seq(',', $.variant))),

        item: $ => choice(
            $.item_constant,
            $.item_module,
            $.item_use,
            $.function_with_body,
            $.item_extern_function,
            $.item_extern_type,
            $.item_trait,
            $.item_impl,
            $.item_impl_alias,
            $.item_struct,
            $.item_enum,
            $.item_type_alias,
            $.item_inline_macro,
        ),
        
        item_list: $ => repeat1($.item),

        attribute: $ => seq(
            '#',
            '[',
            field('attr', $.expr_path),
            optional($.arg_list_parenthesized),
            ']',
        ),
        
        attribute_list: $ => repeat1($.attribute),

        item_module: $ => seq(
            field('attributes', $.attribute_list),
            'mod',
            field('name', $.terminal_identifier),
            field('body', $.maybe_module_body),
        ),
        
        maybe_module_body: $ => choice(
            $.module_body,
            ';',
        ),
        
        module_body: $ => seq(
            '{',
            field('items', $.item_list),
            '}',
        ),
        
        function_declaration: $ => seq(
            'fn',
            field('name', $.terminal_identifier),
            optional($.wrapped_generic_param_list),
            field('signature', $.function_signature),
        ),
        
        item_constant: $ => seq(
            field('attributes', $.attribute_list),
            'const',
            field('name', $.terminal_identifier),
            field('type_clause', $.type_clause),
            '=',
            field('value', $.expr),
            ';',
        ),
        
        function_with_body: $ => seq(
            field('attributes', $.attribute_list),
            field('declaration', $.function_declaration),
            field('body', $.expr_block),
        ),
        
        item_extern_function: $ => seq(
            field('attributes', $.attribute_list),
            'extern',
            field('declaration', $.function_declaration),
            ';',
        ),
        
        item_extern_type: $ => seq(
            field('attributes', $.attribute_list),
            'extern',
            'type',
            field('name', $.terminal_identifier),
            optional($.wrapped_generic_param_list),
            ';',
        ),
        
        item_trait: $ => seq(
            field('attributes', $.attribute_list),
            'trait',
            field('name', $.terminal_identifier),
            optional($.wrapped_generic_param_list),
            field('body', $.maybe_trait_body),
        ),
        
        maybe_trait_body: $ => choice(
            $.trait_body,
            ';',
        ),
        
        trait_body: $ => seq(
            '{',
            field('items', $.trait_item_list),
            '}',
        ),
        
        trait_item_list: $ => repeat1($.trait_item),

        trait_item: $ => choice(
            $.trait_item_function,
        ),
        
        trait_item_function: $ => seq(
            field('attributes', $.attribute_list),
            field('declaration', $.function_declaration),
            field('body', $.maybe_trait_function_body),
        ),
        
        maybe_trait_function_body: $ => choice(
            $.expr_block,
            ';',
        ),
        
        item_impl: $ => seq(
            field('attributes', $.attribute_list),
            'impl',
            field('name', $.terminal_identifier),
            optional($.wrapped_generic_param_list),
            'of',
            field('trait_path', $.expr_path),
            field('body', $.maybe_impl_body),
        ),
        
        item_inline_macro: $ => seq(
            field('attributes', $.attribute_list),
            field('name', $.terminal_identifier),
            '!',
            field('arguments', $.wrapped_arg_list),
            ';',
        ),
        
        maybe_impl_body: $ => choice(
            $.impl_body,
            ';',
        ),
        
        impl_body: $ => seq(
            '{',
            field('items', $.impl_item_list),
            '}',
        ),
        
        impl_item_list: $ => repeat1($.impl_item),

        impl_item: $ => choice(
            $.function_with_body,
            $.item_constant,
            $.item_module,
            $.item_use,
            $.item_extern_function,
            $.item_extern_type,
            $.item_trait,
            $.item_impl,
            $.item_impl_alias,
            $.item_struct,
            $.item_enum,
            $.item_type_alias,
        ),
        
        item_impl_alias: $ => seq(
            field('attributes', $.attribute_list),
            'impl',
            field('name', $.terminal_identifier),
            optional($.wrapped_generic_param_list),
            '=',
            field('impl_path', $.expr_path),
            ';',
        ),
        
        item_struct: $ => seq(
            field('attributes', $.attribute_list),
            'struct',
            field('name', $.terminal_identifier),
            optional($.wrapped_generic_param_list),
            '{',
            field('members', $.member_list),
            '}',
        ),
        
        item_enum: $ => seq(
            field('attributes', $.attribute_list),
            'enum',
            field('name', $.terminal_identifier),
            optional($.wrapped_generic_param_list),
            '{',
            field('variants', $.variant_list),
            '}',
        ),
        
        item_type_alias: $ => seq(
            field('attributes', $.attribute_list),
            'type',
            field('name', $.terminal_identifier),
            optional($.wrapped_generic_param_list),
            '=',
            field('ty', $.expr),
            ';',
        ),
        
        item_use: $ => seq(
            field('attributes', $.attribute_list),
            'use',
            field('use_path', $.use_path),
            ';',
        ),
        
        use_path: $ => choice(
            $.use_path_leaf,
            $.use_path_single,
            $.use_path_multi,
        ),
        
        use_path_leaf: $ => seq(
            field('ident', $.path_segment),
            optional($.alias_clause),
        ),
        
        use_path_single: $ => seq(
            field('ident', $.path_segment),
            '::',
            field('use_path', $.use_path),
        ),
        
        use_path_multi: $ => seq(
            '{',
            field('use_paths', $.use_path_list),
            '}',
        ),
        
        use_path_list: $ => seq($.use_path, repeat(seq(',', $.use_path))),

        alias_clause: $ => seq(
            'as',
            field('alias', $.terminal_identifier),
        ),
        
        generic_arg: $ => choice(
            $.generic_arg_unnamed,
            $.generic_arg_named,
        ),
        
        generic_arg_named: $ => seq(
            field('name', $.terminal_identifier),
            ':',
            field('value', $.generic_arg_value),
        ),
        
        generic_arg_unnamed: $ => seq(
            field('value', $.generic_arg_value),
        ),
        
        generic_arg_value: $ => choice(
            $.generic_arg_value_expr,
            '_',
        ),
        
        generic_arg_value_expr: $ => seq(
            field('expr', $.expr),
        ),
        
        generic_args: $ => seq(
            '<',
            field('generic_args', $.generic_arg_list),
            '>',
        ),
        
        generic_arg_list: $ => seq($.generic_arg, repeat(seq(',', $.generic_arg))),

        wrapped_generic_param_list: $ => seq(
            '<',
            field('generic_params', $.generic_param_list),
            '>',
        ),
        
        generic_param_list: $ => seq($.generic_param, repeat(seq(',', $.generic_param))),

        generic_param: $ => choice(
            $.generic_param_type,
            $.generic_param_const,
            $.generic_param_impl_named,
            $.generic_param_impl_anonymous,
        ),
        
        generic_param_type: $ => seq(
            field('name', $.terminal_identifier),
        ),
        
        generic_param_const: $ => seq(
            'const',
            field('name', $.terminal_identifier),
            ':',
            field('ty', $.expr),
        ),
        
        generic_param_impl_named: $ => seq(
            'impl',
            field('name', $.terminal_identifier),
            ':',
            field('trait_path', $.expr_path),
        ),
        
        generic_param_impl_anonymous: $ => seq(
            '+',
            field('trait_path', $.expr_path),
        ),
        
    }
});
