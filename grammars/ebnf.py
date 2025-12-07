from DHParser.dsl import create_parser

ebnf_grammar = r"""
# EBNF Grammar.
# Directives.
@comment    = /#.*/                # Comments range from a '#'-character to the end of the line.
@whitespace = vertical             # Implicit whitespace, denoted by ~, includes any number of line feeds.
@literalws  = both                 # Literals have implicit whitespace on the right hand side.
@ignorecase = False                # Literals and regular expressions are case-sensitive.
@hide       = atom, factor         # Hide some layers of the grammar for cleaner output.
@drop       = whitespace, strings  # Drop anonymous whitespace and (anonymous) string literals.

# Grammar.
grammar     = { directive | rule }

# Directive.
directive   = "@" IDENTIFIER "=" value
value       = LITERAL | REGEX | list
list        = IDENTIFIER { "," IDENTIFIER }

# EBNF constructs.
rule        = IDENTIFIER "=" expression
expression  = term { "|" term }
term        = factor { factor }
factor      = { WHITESPACE } atom { WHITESPACE } [ lookahead ]
atom        = LITERAL
            | IDENTIFIER ! "="
            | REGEX
            | group
            | optional
            | repetition
group       = "(" expression ")"
optional    = "[" expression "]"
repetition  = "{" expression "}"
lookahead   = ( POSITIVE_LOOKAHEAD
            | NEGATIVE_LOOKAHEAD
            | POSITIVE_LOOKBEHIND
            | NEGATIVE_LOOKBEHIND
            ) factor

# Look ahead / behind.
POSITIVE_LOOKAHEAD  = "&"
NEGATIVE_LOOKAHEAD  = "!"
POSITIVE_LOOKBEHIND = "<-&"
NEGATIVE_LOOKBEHIND = "<-!"

# Whitespace.
WHITESPACE  = "~"

# Tokens.
LITERAL     = /"[^"]*"/~
REGEX       = /\/(?:[^\/\\]|\\.)*\//~
IDENTIFIER  = /[A-Za-z_][A-Za-z_0-9]*/~
"""

# Create a parser from the grammar.
parser = create_parser(ebnf_grammar)


def parse_expr(expr: str):
    tree = parser(expr)
    return tree


if __name__ == "__main__":
    print(parse_expr(ebnf_grammar).as_sxpr())
