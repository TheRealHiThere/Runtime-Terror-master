from numba import jit
from enum import Enum

@jit(nonpython=True)
class Token(Enum):
    TEXT_FUNCTION_DECLARATION = "fn",

    SIGN_BRACKET_CURLY_OPEN   = "{",
    SIGN_BRACKET_CURLY_CLOSE  = "}",

    SIGN_BRACKET_ROUND_OPEN   = "(",
    SIGN_BRACKET_ROUND_CLOSE  = ")",

    SIGN_BRACKET_SQUARE_OPEN  = "[",
    SIGN_BRACKET_SQUARE_CLOSE = "]",