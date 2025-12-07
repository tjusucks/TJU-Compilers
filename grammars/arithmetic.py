from DHParser.dsl import create_parser

arithmetic_grammar = r"""
# Arithmetic Grammar.
# Directives.
@comment     = /#.*/                # Comments range from a '#'-character to the end of the line.
@whitespace  = vertical             # Implicit whitespace, denoted by ~, includes any number of line feeds.
@literalws   = both                 # Literals have implicit whitespace on the right hand side.
@ignorecase  = False                # Literals and regular expressions are case-sensitive.
@drop        = whitespace, strings  # Drop anonymous whitespace and (anonymous) string literals.

# Grammar.
grammar    = ~ { expression }
expression = term  { (PLUS | MINUS) term }
term       = factor { (DIV | MUL) factor }
factor     = [sign] (NUMBER | VARIABLE | group) { VARIABLE | group }
sign       = POSITIVE | NEGATIVE
group      = "(" expression ")"

PLUS       = "+"
MINUS      = "-"
MUL        = "*"
DIV        = "/"

# No implicit whitespace after signs!
POSITIVE   = /[+]/
NEGATIVE   = /[-]/

NUMBER     = /(?:0|(?:[1-9]\d*))(?:\.\d+)?/~
VARIABLE   = /[A-Za-z]/~
"""

# Create a parser from the grammar.
parser = create_parser(arithmetic_grammar)


def parse_expr(expr: str):
    tree = parser(expr)
    return tree


expr = r"""
2 + 3 + 4
2 + 3 + 4
"""


if __name__ == "__main__":
    print(parse_expr(expr).as_sxpr())
