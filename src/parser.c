#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 4
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 74
#define ALIAS_COUNT 0
#define TOKEN_COUNT 73
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 1
#define PRODUCTION_ID_COUNT 1

enum {
  aux_sym_type_suffix_token1 = 1,
  sym_escape_sequence = 2,
  sym_token_single_line_comment = 3,
  sym_terminal_literal_number = 4,
  anon_sym_SQUOTE = 5,
  anon_sym_DQUOTE = 6,
  anon_sym_false = 7,
  anon_sym_true = 8,
  anon_sym_COMMA = 9,
  anon_sym_COLON = 10,
  anon_sym_COLON_COLON = 11,
  anon_sym_LPAREN = 12,
  anon_sym_RPAREN = 13,
  anon_sym_BANG = 14,
  anon_sym_TILDE = 15,
  anon_sym_DASH = 16,
  anon_sym_AT = 17,
  anon_sym_STAR = 18,
  anon_sym_DOT = 19,
  anon_sym_STAR_EQ = 20,
  anon_sym_SLASH = 21,
  anon_sym_SLASH_EQ = 22,
  anon_sym_PERCENT = 23,
  anon_sym_PERCENT_EQ = 24,
  anon_sym_PLUS = 25,
  anon_sym_PLUS_EQ = 26,
  anon_sym_DASH_EQ = 27,
  anon_sym_EQ_EQ = 28,
  anon_sym_BANG_EQ = 29,
  anon_sym_EQ = 30,
  anon_sym_AMP = 31,
  anon_sym_AMP_AMP = 32,
  anon_sym_PIPE = 33,
  anon_sym_PIPE_PIPE = 34,
  anon_sym_CARET = 35,
  anon_sym_LT_EQ = 36,
  anon_sym_LT = 37,
  anon_sym_GT = 38,
  anon_sym_LBRACE = 39,
  anon_sym_RBRACE = 40,
  anon_sym_match = 41,
  anon_sym_EQ_GT = 42,
  anon_sym_if = 43,
  anon_sym_loop = 44,
  anon_sym_else = 45,
  anon_sym_QMARK = 46,
  anon_sym_LBRACK = 47,
  anon_sym_RBRACK = 48,
  anon_sym_DOT_DOT = 49,
  anon_sym__ = 50,
  anon_sym_DASH_GT = 51,
  anon_sym_let = 52,
  anon_sym_SEMI = 53,
  anon_sym_continue = 54,
  anon_sym_return = 55,
  anon_sym_break = 56,
  anon_sym_ref = 57,
  anon_sym_mut = 58,
  anon_sym_nopanic = 59,
  anon_sym_POUND = 60,
  anon_sym_mod = 61,
  anon_sym_fn = 62,
  anon_sym_const = 63,
  anon_sym_extern = 64,
  anon_sym_type = 65,
  anon_sym_trait = 66,
  anon_sym_impl = 67,
  anon_sym_of = 68,
  anon_sym_struct = 69,
  anon_sym_enum = 70,
  anon_sym_use = 71,
  anon_sym_as = 72,
  sym_type_suffix = 73,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [aux_sym_type_suffix_token1] = "type_suffix_token1",
  [sym_escape_sequence] = "escape_sequence",
  [sym_token_single_line_comment] = "token_single_line_comment",
  [sym_terminal_literal_number] = "terminal_literal_number",
  [anon_sym_SQUOTE] = "'",
  [anon_sym_DQUOTE] = "\"",
  [anon_sym_false] = "false",
  [anon_sym_true] = "true",
  [anon_sym_COMMA] = ",",
  [anon_sym_COLON] = ":",
  [anon_sym_COLON_COLON] = "::",
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [anon_sym_BANG] = "!",
  [anon_sym_TILDE] = "~",
  [anon_sym_DASH] = "-",
  [anon_sym_AT] = "@",
  [anon_sym_STAR] = "*",
  [anon_sym_DOT] = ".",
  [anon_sym_STAR_EQ] = "*=",
  [anon_sym_SLASH] = "/",
  [anon_sym_SLASH_EQ] = "/=",
  [anon_sym_PERCENT] = "%",
  [anon_sym_PERCENT_EQ] = "%=",
  [anon_sym_PLUS] = "+",
  [anon_sym_PLUS_EQ] = "+=",
  [anon_sym_DASH_EQ] = "-=",
  [anon_sym_EQ_EQ] = "==",
  [anon_sym_BANG_EQ] = "!=",
  [anon_sym_EQ] = "=",
  [anon_sym_AMP] = "&",
  [anon_sym_AMP_AMP] = "&&",
  [anon_sym_PIPE] = "|",
  [anon_sym_PIPE_PIPE] = "||",
  [anon_sym_CARET] = "^",
  [anon_sym_LT_EQ] = "<=",
  [anon_sym_LT] = "<",
  [anon_sym_GT] = ">",
  [anon_sym_LBRACE] = "{",
  [anon_sym_RBRACE] = "}",
  [anon_sym_match] = "match",
  [anon_sym_EQ_GT] = "=>",
  [anon_sym_if] = "if",
  [anon_sym_loop] = "loop",
  [anon_sym_else] = "else",
  [anon_sym_QMARK] = "\?",
  [anon_sym_LBRACK] = "[",
  [anon_sym_RBRACK] = "]",
  [anon_sym_DOT_DOT] = "..",
  [anon_sym__] = "_",
  [anon_sym_DASH_GT] = "->",
  [anon_sym_let] = "let",
  [anon_sym_SEMI] = ";",
  [anon_sym_continue] = "continue",
  [anon_sym_return] = "return",
  [anon_sym_break] = "break",
  [anon_sym_ref] = "ref",
  [anon_sym_mut] = "mut",
  [anon_sym_nopanic] = "nopanic",
  [anon_sym_POUND] = "#",
  [anon_sym_mod] = "mod",
  [anon_sym_fn] = "fn",
  [anon_sym_const] = "const",
  [anon_sym_extern] = "extern",
  [anon_sym_type] = "type",
  [anon_sym_trait] = "trait",
  [anon_sym_impl] = "impl",
  [anon_sym_of] = "of",
  [anon_sym_struct] = "struct",
  [anon_sym_enum] = "enum",
  [anon_sym_use] = "use",
  [anon_sym_as] = "as",
  [sym_type_suffix] = "type_suffix",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [aux_sym_type_suffix_token1] = aux_sym_type_suffix_token1,
  [sym_escape_sequence] = sym_escape_sequence,
  [sym_token_single_line_comment] = sym_token_single_line_comment,
  [sym_terminal_literal_number] = sym_terminal_literal_number,
  [anon_sym_SQUOTE] = anon_sym_SQUOTE,
  [anon_sym_DQUOTE] = anon_sym_DQUOTE,
  [anon_sym_false] = anon_sym_false,
  [anon_sym_true] = anon_sym_true,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [anon_sym_COLON] = anon_sym_COLON,
  [anon_sym_COLON_COLON] = anon_sym_COLON_COLON,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [anon_sym_BANG] = anon_sym_BANG,
  [anon_sym_TILDE] = anon_sym_TILDE,
  [anon_sym_DASH] = anon_sym_DASH,
  [anon_sym_AT] = anon_sym_AT,
  [anon_sym_STAR] = anon_sym_STAR,
  [anon_sym_DOT] = anon_sym_DOT,
  [anon_sym_STAR_EQ] = anon_sym_STAR_EQ,
  [anon_sym_SLASH] = anon_sym_SLASH,
  [anon_sym_SLASH_EQ] = anon_sym_SLASH_EQ,
  [anon_sym_PERCENT] = anon_sym_PERCENT,
  [anon_sym_PERCENT_EQ] = anon_sym_PERCENT_EQ,
  [anon_sym_PLUS] = anon_sym_PLUS,
  [anon_sym_PLUS_EQ] = anon_sym_PLUS_EQ,
  [anon_sym_DASH_EQ] = anon_sym_DASH_EQ,
  [anon_sym_EQ_EQ] = anon_sym_EQ_EQ,
  [anon_sym_BANG_EQ] = anon_sym_BANG_EQ,
  [anon_sym_EQ] = anon_sym_EQ,
  [anon_sym_AMP] = anon_sym_AMP,
  [anon_sym_AMP_AMP] = anon_sym_AMP_AMP,
  [anon_sym_PIPE] = anon_sym_PIPE,
  [anon_sym_PIPE_PIPE] = anon_sym_PIPE_PIPE,
  [anon_sym_CARET] = anon_sym_CARET,
  [anon_sym_LT_EQ] = anon_sym_LT_EQ,
  [anon_sym_LT] = anon_sym_LT,
  [anon_sym_GT] = anon_sym_GT,
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [anon_sym_match] = anon_sym_match,
  [anon_sym_EQ_GT] = anon_sym_EQ_GT,
  [anon_sym_if] = anon_sym_if,
  [anon_sym_loop] = anon_sym_loop,
  [anon_sym_else] = anon_sym_else,
  [anon_sym_QMARK] = anon_sym_QMARK,
  [anon_sym_LBRACK] = anon_sym_LBRACK,
  [anon_sym_RBRACK] = anon_sym_RBRACK,
  [anon_sym_DOT_DOT] = anon_sym_DOT_DOT,
  [anon_sym__] = anon_sym__,
  [anon_sym_DASH_GT] = anon_sym_DASH_GT,
  [anon_sym_let] = anon_sym_let,
  [anon_sym_SEMI] = anon_sym_SEMI,
  [anon_sym_continue] = anon_sym_continue,
  [anon_sym_return] = anon_sym_return,
  [anon_sym_break] = anon_sym_break,
  [anon_sym_ref] = anon_sym_ref,
  [anon_sym_mut] = anon_sym_mut,
  [anon_sym_nopanic] = anon_sym_nopanic,
  [anon_sym_POUND] = anon_sym_POUND,
  [anon_sym_mod] = anon_sym_mod,
  [anon_sym_fn] = anon_sym_fn,
  [anon_sym_const] = anon_sym_const,
  [anon_sym_extern] = anon_sym_extern,
  [anon_sym_type] = anon_sym_type,
  [anon_sym_trait] = anon_sym_trait,
  [anon_sym_impl] = anon_sym_impl,
  [anon_sym_of] = anon_sym_of,
  [anon_sym_struct] = anon_sym_struct,
  [anon_sym_enum] = anon_sym_enum,
  [anon_sym_use] = anon_sym_use,
  [anon_sym_as] = anon_sym_as,
  [sym_type_suffix] = sym_type_suffix,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [aux_sym_type_suffix_token1] = {
    .visible = false,
    .named = false,
  },
  [sym_escape_sequence] = {
    .visible = true,
    .named = true,
  },
  [sym_token_single_line_comment] = {
    .visible = true,
    .named = true,
  },
  [sym_terminal_literal_number] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_SQUOTE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DQUOTE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_false] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_true] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COMMA] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COLON] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COLON_COLON] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_BANG] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_TILDE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_AT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_STAR] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DOT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_STAR_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SLASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SLASH_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PERCENT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PERCENT_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PLUS] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PLUS_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DASH_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_EQ_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_BANG_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_AMP] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_AMP_AMP] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PIPE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PIPE_PIPE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_CARET] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LT_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_GT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_match] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_EQ_GT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_if] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_loop] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_else] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_QMARK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DOT_DOT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym__] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DASH_GT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_let] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SEMI] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_continue] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_return] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_break] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ref] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_mut] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_nopanic] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_POUND] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_mod] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_fn] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_const] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_extern] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_type] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_trait] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_impl] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_of] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_struct] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_enum] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_use] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_as] = {
    .visible = true,
    .named = false,
  },
  [sym_type_suffix] = {
    .visible = true,
    .named = true,
  },
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(72);
      if (lookahead == '!') ADVANCE(95);
      if (lookahead == '"') ADVANCE(87);
      if (lookahead == '#') ADVANCE(141);
      if (lookahead == '%') ADVANCE(104);
      if (lookahead == '&') ADVANCE(112);
      if (lookahead == '\'') ADVANCE(86);
      if (lookahead == '(') ADVANCE(93);
      if (lookahead == ')') ADVANCE(94);
      if (lookahead == '*') ADVANCE(99);
      if (lookahead == '+') ADVANCE(106);
      if (lookahead == ',') ADVANCE(90);
      if (lookahead == '-') ADVANCE(97);
      if (lookahead == '.') ADVANCE(100);
      if (lookahead == '/') ADVANCE(102);
      if (lookahead == '0') ADVANCE(76);
      if (lookahead == ':') ADVANCE(91);
      if (lookahead == ';') ADVANCE(134);
      if (lookahead == '<') ADVANCE(118);
      if (lookahead == '=') ADVANCE(111);
      if (lookahead == '>') ADVANCE(119);
      if (lookahead == '?') ADVANCE(127);
      if (lookahead == '@') ADVANCE(98);
      if (lookahead == '[') ADVANCE(128);
      if (lookahead == '\\') ADVANCE(71);
      if (lookahead == ']') ADVANCE(129);
      if (lookahead == '^') ADVANCE(116);
      if (lookahead == '_') ADVANCE(131);
      if (lookahead == 'a') ADVANCE(50);
      if (lookahead == 'b') ADVANCE(46);
      if (lookahead == 'c') ADVANCE(38);
      if (lookahead == 'e') ADVANCE(30);
      if (lookahead == 'f') ADVANCE(2);
      if (lookahead == 'i') ADVANCE(21);
      if (lookahead == 'l') ADVANCE(20);
      if (lookahead == 'm') ADVANCE(6);
      if (lookahead == 'n') ADVANCE(39);
      if (lookahead == 'o') ADVANCE(22);
      if (lookahead == 'r') ADVANCE(11);
      if (lookahead == 's') ADVANCE(61);
      if (lookahead == 't') ADVANCE(45);
      if (lookahead == 'u') ADVANCE(51);
      if (lookahead == '{') ADVANCE(120);
      if (lookahead == '|') ADVANCE(114);
      if (lookahead == '}') ADVANCE(121);
      if (lookahead == '~') ADVANCE(96);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(77);
      END_STATE();
    case 1:
      if (lookahead == '_') ADVANCE(70);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(1)
      END_STATE();
    case 2:
      if (lookahead == 'a') ADVANCE(31);
      if (lookahead == 'n') ADVANCE(143);
      END_STATE();
    case 3:
      if (lookahead == 'a') ADVANCE(27);
      if (lookahead == 'u') ADVANCE(15);
      END_STATE();
    case 4:
      if (lookahead == 'a') ADVANCE(28);
      END_STATE();
    case 5:
      if (lookahead == 'a') ADVANCE(36);
      END_STATE();
    case 6:
      if (lookahead == 'a') ADVANCE(56);
      if (lookahead == 'o') ADVANCE(10);
      if (lookahead == 'u') ADVANCE(57);
      END_STATE();
    case 7:
      if (lookahead == 'c') ADVANCE(24);
      END_STATE();
    case 8:
      if (lookahead == 'c') ADVANCE(140);
      END_STATE();
    case 9:
      if (lookahead == 'c') ADVANCE(60);
      END_STATE();
    case 10:
      if (lookahead == 'd') ADVANCE(142);
      END_STATE();
    case 11:
      if (lookahead == 'e') ADVANCE(23);
      END_STATE();
    case 12:
      if (lookahead == 'e') ADVANCE(4);
      END_STATE();
    case 13:
      if (lookahead == 'e') ADVANCE(152);
      END_STATE();
    case 14:
      if (lookahead == 'e') ADVANCE(126);
      END_STATE();
    case 15:
      if (lookahead == 'e') ADVANCE(89);
      END_STATE();
    case 16:
      if (lookahead == 'e') ADVANCE(146);
      END_STATE();
    case 17:
      if (lookahead == 'e') ADVANCE(88);
      END_STATE();
    case 18:
      if (lookahead == 'e') ADVANCE(135);
      END_STATE();
    case 19:
      if (lookahead == 'e') ADVANCE(48);
      END_STATE();
    case 20:
      if (lookahead == 'e') ADVANCE(55);
      if (lookahead == 'o') ADVANCE(40);
      END_STATE();
    case 21:
      if (lookahead == 'f') ADVANCE(124);
      if (lookahead == 'm') ADVANCE(42);
      END_STATE();
    case 22:
      if (lookahead == 'f') ADVANCE(149);
      END_STATE();
    case 23:
      if (lookahead == 'f') ADVANCE(138);
      if (lookahead == 't') ADVANCE(66);
      END_STATE();
    case 24:
      if (lookahead == 'h') ADVANCE(122);
      END_STATE();
    case 25:
      if (lookahead == 'i') ADVANCE(37);
      END_STATE();
    case 26:
      if (lookahead == 'i') ADVANCE(8);
      END_STATE();
    case 27:
      if (lookahead == 'i') ADVANCE(59);
      END_STATE();
    case 28:
      if (lookahead == 'k') ADVANCE(137);
      END_STATE();
    case 29:
      if (lookahead == 'l') ADVANCE(148);
      END_STATE();
    case 30:
      if (lookahead == 'l') ADVANCE(52);
      if (lookahead == 'n') ADVANCE(63);
      if (lookahead == 'x') ADVANCE(62);
      END_STATE();
    case 31:
      if (lookahead == 'l') ADVANCE(54);
      END_STATE();
    case 32:
      if (lookahead == 'm') ADVANCE(151);
      END_STATE();
    case 33:
      if (lookahead == 'n') ADVANCE(53);
      END_STATE();
    case 34:
      if (lookahead == 'n') ADVANCE(145);
      END_STATE();
    case 35:
      if (lookahead == 'n') ADVANCE(136);
      END_STATE();
    case 36:
      if (lookahead == 'n') ADVANCE(26);
      END_STATE();
    case 37:
      if (lookahead == 'n') ADVANCE(65);
      END_STATE();
    case 38:
      if (lookahead == 'o') ADVANCE(33);
      END_STATE();
    case 39:
      if (lookahead == 'o') ADVANCE(43);
      END_STATE();
    case 40:
      if (lookahead == 'o') ADVANCE(41);
      END_STATE();
    case 41:
      if (lookahead == 'p') ADVANCE(125);
      END_STATE();
    case 42:
      if (lookahead == 'p') ADVANCE(29);
      END_STATE();
    case 43:
      if (lookahead == 'p') ADVANCE(5);
      END_STATE();
    case 44:
      if (lookahead == 'p') ADVANCE(16);
      END_STATE();
    case 45:
      if (lookahead == 'r') ADVANCE(3);
      if (lookahead == 'y') ADVANCE(44);
      END_STATE();
    case 46:
      if (lookahead == 'r') ADVANCE(12);
      END_STATE();
    case 47:
      if (lookahead == 'r') ADVANCE(64);
      END_STATE();
    case 48:
      if (lookahead == 'r') ADVANCE(34);
      END_STATE();
    case 49:
      if (lookahead == 'r') ADVANCE(35);
      END_STATE();
    case 50:
      if (lookahead == 's') ADVANCE(153);
      END_STATE();
    case 51:
      if (lookahead == 's') ADVANCE(13);
      END_STATE();
    case 52:
      if (lookahead == 's') ADVANCE(14);
      END_STATE();
    case 53:
      if (lookahead == 's') ADVANCE(58);
      if (lookahead == 't') ADVANCE(25);
      END_STATE();
    case 54:
      if (lookahead == 's') ADVANCE(17);
      END_STATE();
    case 55:
      if (lookahead == 't') ADVANCE(133);
      END_STATE();
    case 56:
      if (lookahead == 't') ADVANCE(7);
      END_STATE();
    case 57:
      if (lookahead == 't') ADVANCE(139);
      END_STATE();
    case 58:
      if (lookahead == 't') ADVANCE(144);
      END_STATE();
    case 59:
      if (lookahead == 't') ADVANCE(147);
      END_STATE();
    case 60:
      if (lookahead == 't') ADVANCE(150);
      END_STATE();
    case 61:
      if (lookahead == 't') ADVANCE(47);
      END_STATE();
    case 62:
      if (lookahead == 't') ADVANCE(19);
      END_STATE();
    case 63:
      if (lookahead == 'u') ADVANCE(32);
      END_STATE();
    case 64:
      if (lookahead == 'u') ADVANCE(9);
      END_STATE();
    case 65:
      if (lookahead == 'u') ADVANCE(18);
      END_STATE();
    case 66:
      if (lookahead == 'u') ADVANCE(49);
      END_STATE();
    case 67:
      if (lookahead == '0' ||
          lookahead == '1' ||
          lookahead == '_') ADVANCE(79);
      END_STATE();
    case 68:
      if (('0' <= lookahead && lookahead <= '7') ||
          lookahead == '_') ADVANCE(81);
      END_STATE();
    case 69:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(83);
      END_STATE();
    case 70:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(73);
      END_STATE();
    case 71:
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(74);
      END_STATE();
    case 72:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 73:
      ACCEPT_TOKEN(aux_sym_type_suffix_token1);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(73);
      END_STATE();
    case 74:
      ACCEPT_TOKEN(sym_escape_sequence);
      END_STATE();
    case 75:
      ACCEPT_TOKEN(sym_token_single_line_comment);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(75);
      END_STATE();
    case 76:
      ACCEPT_TOKEN(sym_terminal_literal_number);
      if (lookahead == '_') ADVANCE(78);
      if (lookahead == 'b') ADVANCE(67);
      if (lookahead == 'o') ADVANCE(68);
      if (lookahead == 'x') ADVANCE(69);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(77);
      END_STATE();
    case 77:
      ACCEPT_TOKEN(sym_terminal_literal_number);
      if (lookahead == '_') ADVANCE(78);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(77);
      END_STATE();
    case 78:
      ACCEPT_TOKEN(sym_terminal_literal_number);
      if (lookahead == '_') ADVANCE(78);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(78);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(85);
      END_STATE();
    case 79:
      ACCEPT_TOKEN(sym_terminal_literal_number);
      if (lookahead == '_') ADVANCE(80);
      if (lookahead == '0' ||
          lookahead == '1') ADVANCE(79);
      END_STATE();
    case 80:
      ACCEPT_TOKEN(sym_terminal_literal_number);
      if (lookahead == '_') ADVANCE(80);
      if (lookahead == '0' ||
          lookahead == '1') ADVANCE(80);
      if (('2' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(85);
      END_STATE();
    case 81:
      ACCEPT_TOKEN(sym_terminal_literal_number);
      if (lookahead == '_') ADVANCE(82);
      if (('0' <= lookahead && lookahead <= '7')) ADVANCE(81);
      END_STATE();
    case 82:
      ACCEPT_TOKEN(sym_terminal_literal_number);
      if (lookahead == '_') ADVANCE(82);
      if (('0' <= lookahead && lookahead <= '7')) ADVANCE(82);
      if (lookahead == '8' ||
          lookahead == '9' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(85);
      END_STATE();
    case 83:
      ACCEPT_TOKEN(sym_terminal_literal_number);
      if (lookahead == '_') ADVANCE(84);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(83);
      END_STATE();
    case 84:
      ACCEPT_TOKEN(sym_terminal_literal_number);
      if (lookahead == '_') ADVANCE(84);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(84);
      if (('G' <= lookahead && lookahead <= 'Z') ||
          ('g' <= lookahead && lookahead <= 'z')) ADVANCE(85);
      END_STATE();
    case 85:
      ACCEPT_TOKEN(sym_terminal_literal_number);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(85);
      END_STATE();
    case 86:
      ACCEPT_TOKEN(anon_sym_SQUOTE);
      END_STATE();
    case 87:
      ACCEPT_TOKEN(anon_sym_DQUOTE);
      END_STATE();
    case 88:
      ACCEPT_TOKEN(anon_sym_false);
      END_STATE();
    case 89:
      ACCEPT_TOKEN(anon_sym_true);
      END_STATE();
    case 90:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 91:
      ACCEPT_TOKEN(anon_sym_COLON);
      if (lookahead == ':') ADVANCE(92);
      END_STATE();
    case 92:
      ACCEPT_TOKEN(anon_sym_COLON_COLON);
      END_STATE();
    case 93:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 94:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 95:
      ACCEPT_TOKEN(anon_sym_BANG);
      if (lookahead == '=') ADVANCE(110);
      END_STATE();
    case 96:
      ACCEPT_TOKEN(anon_sym_TILDE);
      END_STATE();
    case 97:
      ACCEPT_TOKEN(anon_sym_DASH);
      if (lookahead == '=') ADVANCE(108);
      if (lookahead == '>') ADVANCE(132);
      END_STATE();
    case 98:
      ACCEPT_TOKEN(anon_sym_AT);
      END_STATE();
    case 99:
      ACCEPT_TOKEN(anon_sym_STAR);
      if (lookahead == '=') ADVANCE(101);
      END_STATE();
    case 100:
      ACCEPT_TOKEN(anon_sym_DOT);
      if (lookahead == '.') ADVANCE(130);
      END_STATE();
    case 101:
      ACCEPT_TOKEN(anon_sym_STAR_EQ);
      END_STATE();
    case 102:
      ACCEPT_TOKEN(anon_sym_SLASH);
      if (lookahead == '/') ADVANCE(75);
      if (lookahead == '=') ADVANCE(103);
      END_STATE();
    case 103:
      ACCEPT_TOKEN(anon_sym_SLASH_EQ);
      END_STATE();
    case 104:
      ACCEPT_TOKEN(anon_sym_PERCENT);
      if (lookahead == '=') ADVANCE(105);
      END_STATE();
    case 105:
      ACCEPT_TOKEN(anon_sym_PERCENT_EQ);
      END_STATE();
    case 106:
      ACCEPT_TOKEN(anon_sym_PLUS);
      if (lookahead == '=') ADVANCE(107);
      END_STATE();
    case 107:
      ACCEPT_TOKEN(anon_sym_PLUS_EQ);
      END_STATE();
    case 108:
      ACCEPT_TOKEN(anon_sym_DASH_EQ);
      END_STATE();
    case 109:
      ACCEPT_TOKEN(anon_sym_EQ_EQ);
      END_STATE();
    case 110:
      ACCEPT_TOKEN(anon_sym_BANG_EQ);
      END_STATE();
    case 111:
      ACCEPT_TOKEN(anon_sym_EQ);
      if (lookahead == '=') ADVANCE(109);
      if (lookahead == '>') ADVANCE(123);
      END_STATE();
    case 112:
      ACCEPT_TOKEN(anon_sym_AMP);
      if (lookahead == '&') ADVANCE(113);
      END_STATE();
    case 113:
      ACCEPT_TOKEN(anon_sym_AMP_AMP);
      END_STATE();
    case 114:
      ACCEPT_TOKEN(anon_sym_PIPE);
      if (lookahead == '|') ADVANCE(115);
      END_STATE();
    case 115:
      ACCEPT_TOKEN(anon_sym_PIPE_PIPE);
      END_STATE();
    case 116:
      ACCEPT_TOKEN(anon_sym_CARET);
      END_STATE();
    case 117:
      ACCEPT_TOKEN(anon_sym_LT_EQ);
      END_STATE();
    case 118:
      ACCEPT_TOKEN(anon_sym_LT);
      if (lookahead == '=') ADVANCE(117);
      END_STATE();
    case 119:
      ACCEPT_TOKEN(anon_sym_GT);
      END_STATE();
    case 120:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 121:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 122:
      ACCEPT_TOKEN(anon_sym_match);
      END_STATE();
    case 123:
      ACCEPT_TOKEN(anon_sym_EQ_GT);
      END_STATE();
    case 124:
      ACCEPT_TOKEN(anon_sym_if);
      END_STATE();
    case 125:
      ACCEPT_TOKEN(anon_sym_loop);
      END_STATE();
    case 126:
      ACCEPT_TOKEN(anon_sym_else);
      END_STATE();
    case 127:
      ACCEPT_TOKEN(anon_sym_QMARK);
      END_STATE();
    case 128:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      END_STATE();
    case 129:
      ACCEPT_TOKEN(anon_sym_RBRACK);
      END_STATE();
    case 130:
      ACCEPT_TOKEN(anon_sym_DOT_DOT);
      END_STATE();
    case 131:
      ACCEPT_TOKEN(anon_sym__);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(73);
      END_STATE();
    case 132:
      ACCEPT_TOKEN(anon_sym_DASH_GT);
      END_STATE();
    case 133:
      ACCEPT_TOKEN(anon_sym_let);
      END_STATE();
    case 134:
      ACCEPT_TOKEN(anon_sym_SEMI);
      END_STATE();
    case 135:
      ACCEPT_TOKEN(anon_sym_continue);
      END_STATE();
    case 136:
      ACCEPT_TOKEN(anon_sym_return);
      END_STATE();
    case 137:
      ACCEPT_TOKEN(anon_sym_break);
      END_STATE();
    case 138:
      ACCEPT_TOKEN(anon_sym_ref);
      END_STATE();
    case 139:
      ACCEPT_TOKEN(anon_sym_mut);
      END_STATE();
    case 140:
      ACCEPT_TOKEN(anon_sym_nopanic);
      END_STATE();
    case 141:
      ACCEPT_TOKEN(anon_sym_POUND);
      END_STATE();
    case 142:
      ACCEPT_TOKEN(anon_sym_mod);
      END_STATE();
    case 143:
      ACCEPT_TOKEN(anon_sym_fn);
      END_STATE();
    case 144:
      ACCEPT_TOKEN(anon_sym_const);
      END_STATE();
    case 145:
      ACCEPT_TOKEN(anon_sym_extern);
      END_STATE();
    case 146:
      ACCEPT_TOKEN(anon_sym_type);
      END_STATE();
    case 147:
      ACCEPT_TOKEN(anon_sym_trait);
      END_STATE();
    case 148:
      ACCEPT_TOKEN(anon_sym_impl);
      END_STATE();
    case 149:
      ACCEPT_TOKEN(anon_sym_of);
      END_STATE();
    case 150:
      ACCEPT_TOKEN(anon_sym_struct);
      END_STATE();
    case 151:
      ACCEPT_TOKEN(anon_sym_enum);
      END_STATE();
    case 152:
      ACCEPT_TOKEN(anon_sym_use);
      END_STATE();
    case 153:
      ACCEPT_TOKEN(anon_sym_as);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 1},
  [2] = {.lex_state = 0},
  [3] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [aux_sym_type_suffix_token1] = ACTIONS(1),
    [sym_escape_sequence] = ACTIONS(1),
    [sym_token_single_line_comment] = ACTIONS(1),
    [sym_terminal_literal_number] = ACTIONS(1),
    [anon_sym_SQUOTE] = ACTIONS(1),
    [anon_sym_DQUOTE] = ACTIONS(1),
    [anon_sym_false] = ACTIONS(1),
    [anon_sym_true] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [anon_sym_COLON_COLON] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [anon_sym_BANG] = ACTIONS(1),
    [anon_sym_TILDE] = ACTIONS(1),
    [anon_sym_DASH] = ACTIONS(1),
    [anon_sym_AT] = ACTIONS(1),
    [anon_sym_STAR] = ACTIONS(1),
    [anon_sym_DOT] = ACTIONS(1),
    [anon_sym_STAR_EQ] = ACTIONS(1),
    [anon_sym_SLASH] = ACTIONS(1),
    [anon_sym_SLASH_EQ] = ACTIONS(1),
    [anon_sym_PERCENT] = ACTIONS(1),
    [anon_sym_PERCENT_EQ] = ACTIONS(1),
    [anon_sym_PLUS] = ACTIONS(1),
    [anon_sym_PLUS_EQ] = ACTIONS(1),
    [anon_sym_DASH_EQ] = ACTIONS(1),
    [anon_sym_EQ_EQ] = ACTIONS(1),
    [anon_sym_BANG_EQ] = ACTIONS(1),
    [anon_sym_EQ] = ACTIONS(1),
    [anon_sym_AMP] = ACTIONS(1),
    [anon_sym_AMP_AMP] = ACTIONS(1),
    [anon_sym_PIPE] = ACTIONS(1),
    [anon_sym_PIPE_PIPE] = ACTIONS(1),
    [anon_sym_CARET] = ACTIONS(1),
    [anon_sym_LT_EQ] = ACTIONS(1),
    [anon_sym_LT] = ACTIONS(1),
    [anon_sym_GT] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [anon_sym_match] = ACTIONS(1),
    [anon_sym_EQ_GT] = ACTIONS(1),
    [anon_sym_if] = ACTIONS(1),
    [anon_sym_loop] = ACTIONS(1),
    [anon_sym_else] = ACTIONS(1),
    [anon_sym_QMARK] = ACTIONS(1),
    [anon_sym_LBRACK] = ACTIONS(1),
    [anon_sym_RBRACK] = ACTIONS(1),
    [anon_sym_DOT_DOT] = ACTIONS(1),
    [anon_sym__] = ACTIONS(1),
    [anon_sym_DASH_GT] = ACTIONS(1),
    [anon_sym_let] = ACTIONS(1),
    [anon_sym_SEMI] = ACTIONS(1),
    [anon_sym_continue] = ACTIONS(1),
    [anon_sym_return] = ACTIONS(1),
    [anon_sym_break] = ACTIONS(1),
    [anon_sym_ref] = ACTIONS(1),
    [anon_sym_mut] = ACTIONS(1),
    [anon_sym_nopanic] = ACTIONS(1),
    [anon_sym_POUND] = ACTIONS(1),
    [anon_sym_mod] = ACTIONS(1),
    [anon_sym_fn] = ACTIONS(1),
    [anon_sym_const] = ACTIONS(1),
    [anon_sym_extern] = ACTIONS(1),
    [anon_sym_type] = ACTIONS(1),
    [anon_sym_trait] = ACTIONS(1),
    [anon_sym_impl] = ACTIONS(1),
    [anon_sym_of] = ACTIONS(1),
    [anon_sym_struct] = ACTIONS(1),
    [anon_sym_enum] = ACTIONS(1),
    [anon_sym_use] = ACTIONS(1),
    [anon_sym_as] = ACTIONS(1),
  },
  [1] = {
    [sym_type_suffix] = STATE(3),
    [aux_sym_type_suffix_token1] = ACTIONS(3),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 1,
    ACTIONS(5), 1,
      ts_builtin_sym_end,
  [4] = 1,
    ACTIONS(7), 1,
      ts_builtin_sym_end,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 4,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_type_suffix, 1),
  [7] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_cairo(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
