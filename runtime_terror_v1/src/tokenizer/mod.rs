#![allow(non_camel_case_types)]
#![allow(unused)]

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum TEXT_TOKEN {
    TEXT_FUNCTION_DECLARATION_1,
    TEXT_FUNCTION_DECLARATION_2,

    TEXT_CLASS_DECLARATION_1,
    TEXT_CLASS_DECLARATION_2,

    TEXT_ENUM_DECLARATION_1,
    TEXT_ENUM_DECLARATION_2,
    TEXT_ENUM_DECLARATION_3,
    TEXT_ENUM_DECLARATION_4,

    TEXT_VAR_DECLARATION_1,
    TEXT_VAR_DECLARATION_2,
    TEXT_VAR_DECLARATION_3,

    TEXT_MUT_DECLARATION_1,
    TEXT_MUT_DECLARATION_2,
    TEXT_MUT_DECLARATION_3,

    TEXT_PUB_DECLARATION_2,
    TEXT_PUB_DECLARATION_1,
    TEXT_PUB_DECLARATION_3,

    TEXT_CONST_DECLARATION_1,
    TEXT_CONST_DECLARATION_2,
    TEXT_CONST_DECLARATION_3,
    TEXT_CONST_DECLARATION_4,
    TEXT_CONST_DECLARATION_5,

    TEXT_TYPE_DECLARATION_1,
    TEXT_TYPE_DECLARATION_2,
    TEXT_TYPE_DECLARATION_3,
    TEXT_TYPE_DECLARATION_4,

    TEXT_SWITCH_DECLARATION_1,
    TEXT_SWITCH_DECLARATION_2,
    TEXT_SWITCH_DECLARATION_3,
    TEXT_SWITCH_DECLARATION_4,
    TEXT_SWITCH_DECLARATION_5,
    TEXT_SWITCH_DECLARATION_6,

    TEXT_RETURN_DECLARATION_1,
    TEXT_RETURN_DECLARATION_2,
    TEXT_RETURN_DECLARATION_3,
    TEXT_RETURN_DECLARATION_4,
    TEXT_RETURN_DECLARATION_5,
    TEXT_RETURN_DECLARATION_6,

    TEXT_DERIVE_DECLARATION_1,
    TEXT_DERIVE_DECLARATION_2,
    TEXT_DERIVE_DECLARATION_3,
    TEXT_DERIVE_DECLARATION_4,
    TEXT_DERIVE_DECLARATION_5,
    TEXT_DERIVE_DECLARATION_6,

    TEXT_CONSTR_DECLARATION_1,
    TEXT_CONSTR_DECLARATION_2,
    TEXT_CONSTR_DECLARATION_3,
    TEXT_CONSTR_DECLARATION_4,
    TEXT_CONSTR_DECLARATION_5,
    TEXT_CONSTR_DECLARATION_6,

    TEXT_STATIC_DECLARATION_1,
    TEXT_STATIC_DECLARATION_2,
    TEXT_STATIC_DECLARATION_3,
    TEXT_STATIC_DECLARATION_4,
    TEXT_STATIC_DECLARATION_5,
    TEXT_STATIC_DECLARATION_6,

    TEXT_UNSAFE_DECLARATION_1,
    TEXT_UNSAFE_DECLARATION_2,
    TEXT_UNSAFE_DECLARATION_3,
    TEXT_UNSAFE_DECLARATION_4,
    TEXT_UNSAFE_DECLARATION_5,
    TEXT_UNSAFE_DECLARATION_6,

    TEXT_OTHER(char),
}
impl Into<char> for TEXT_TOKEN {
    fn into(self) -> char {
        match self {
            TEXT_TOKEN::TEXT_FUNCTION_DECLARATION_1 => 'f',
            TEXT_TOKEN::TEXT_FUNCTION_DECLARATION_2 => 'n',
            TEXT_TOKEN::TEXT_CLASS_DECLARATION_1 => 'c',
            TEXT_TOKEN::TEXT_CLASS_DECLARATION_2 => 'l',
            TEXT_TOKEN::TEXT_ENUM_DECLARATION_1 => 'e',
            TEXT_TOKEN::TEXT_ENUM_DECLARATION_2 => 'n',
            TEXT_TOKEN::TEXT_ENUM_DECLARATION_3 => 'u',
            TEXT_TOKEN::TEXT_ENUM_DECLARATION_4 => 'm',
            TEXT_TOKEN::TEXT_VAR_DECLARATION_1 => 'v',
            TEXT_TOKEN::TEXT_VAR_DECLARATION_2 => 'a',
            TEXT_TOKEN::TEXT_VAR_DECLARATION_3 => 'r',
            TEXT_TOKEN::TEXT_MUT_DECLARATION_1 => 'm',
            TEXT_TOKEN::TEXT_MUT_DECLARATION_2 => 'u',
            TEXT_TOKEN::TEXT_MUT_DECLARATION_3 => 't',
            TEXT_TOKEN::TEXT_PUB_DECLARATION_2 => 'p',
            TEXT_TOKEN::TEXT_PUB_DECLARATION_1 => 'u',
            TEXT_TOKEN::TEXT_PUB_DECLARATION_3 => 'b',
            TEXT_TOKEN::TEXT_CONST_DECLARATION_1 => 'c',
            TEXT_TOKEN::TEXT_CONST_DECLARATION_2 => 'o',
            TEXT_TOKEN::TEXT_CONST_DECLARATION_3 => 'n',
            TEXT_TOKEN::TEXT_CONST_DECLARATION_4 => 's',
            TEXT_TOKEN::TEXT_CONST_DECLARATION_5 => 't',
            TEXT_TOKEN::TEXT_TYPE_DECLARATION_1 => 't',
            TEXT_TOKEN::TEXT_TYPE_DECLARATION_2 => 'y',
            TEXT_TOKEN::TEXT_TYPE_DECLARATION_3 => 'p',
            TEXT_TOKEN::TEXT_TYPE_DECLARATION_4 => 'e',
            TEXT_TOKEN::TEXT_SWITCH_DECLARATION_1 => 's',
            TEXT_TOKEN::TEXT_SWITCH_DECLARATION_2 => 'w',
            TEXT_TOKEN::TEXT_SWITCH_DECLARATION_3 => 'i',
            TEXT_TOKEN::TEXT_SWITCH_DECLARATION_4 => 't',
            TEXT_TOKEN::TEXT_SWITCH_DECLARATION_5 => 'c',
            TEXT_TOKEN::TEXT_SWITCH_DECLARATION_6 => 'h',
            TEXT_TOKEN::TEXT_RETURN_DECLARATION_1 => 'r',
            TEXT_TOKEN::TEXT_RETURN_DECLARATION_2 => 'e',
            TEXT_TOKEN::TEXT_RETURN_DECLARATION_3 => 't',
            TEXT_TOKEN::TEXT_RETURN_DECLARATION_4 => 'u',
            TEXT_TOKEN::TEXT_RETURN_DECLARATION_5 => 'r',
            TEXT_TOKEN::TEXT_RETURN_DECLARATION_6 => 'n',
            TEXT_TOKEN::TEXT_DERIVE_DECLARATION_1 => 'd',
            TEXT_TOKEN::TEXT_DERIVE_DECLARATION_2 => 'e',
            TEXT_TOKEN::TEXT_DERIVE_DECLARATION_3 => 'r',
            TEXT_TOKEN::TEXT_DERIVE_DECLARATION_4 => 'i',
            TEXT_TOKEN::TEXT_DERIVE_DECLARATION_5 => 'v',
            TEXT_TOKEN::TEXT_DERIVE_DECLARATION_6 => 'e',
            TEXT_TOKEN::TEXT_CONSTR_DECLARATION_1 => 'c',
            TEXT_TOKEN::TEXT_CONSTR_DECLARATION_2 => 'o',
            TEXT_TOKEN::TEXT_CONSTR_DECLARATION_3 => 'n',
            TEXT_TOKEN::TEXT_CONSTR_DECLARATION_4 => 's',
            TEXT_TOKEN::TEXT_CONSTR_DECLARATION_5 => 't',
            TEXT_TOKEN::TEXT_CONSTR_DECLARATION_6 => 'r',
            TEXT_TOKEN::TEXT_STATIC_DECLARATION_1 => 's',
            TEXT_TOKEN::TEXT_STATIC_DECLARATION_2 => 't',
            TEXT_TOKEN::TEXT_STATIC_DECLARATION_3 => 'a',
            TEXT_TOKEN::TEXT_STATIC_DECLARATION_4 => 't',
            TEXT_TOKEN::TEXT_STATIC_DECLARATION_5 => 'i',
            TEXT_TOKEN::TEXT_STATIC_DECLARATION_6 => 'c',
            TEXT_TOKEN::TEXT_UNSAFE_DECLARATION_1 => 'u',
            TEXT_TOKEN::TEXT_UNSAFE_DECLARATION_2 => 'n',
            TEXT_TOKEN::TEXT_UNSAFE_DECLARATION_3 => 's',
            TEXT_TOKEN::TEXT_UNSAFE_DECLARATION_4 => 'a',
            TEXT_TOKEN::TEXT_UNSAFE_DECLARATION_5 => 'f',
            TEXT_TOKEN::TEXT_UNSAFE_DECLARATION_6 => 'e',
            TEXT_TOKEN::TEXT_OTHER(v) => v,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum SIGN_TOKEN {
    SIGN_BRACKET_CURLY_OPEN,
    SIGN_BRACKET_CURLY_CLOSE,
    SIGN_BRACKET_ROUND_OPEN,
    SIGN_BRACKET_ROUND_CLOSE,
    SIGN_BRACKET_SQUARE_OPEN,
    SIGN_BRACKET_SQUARE_CLOSE,
    SIGN_LESS_THAN,
    SIGN_GREATER_THAN,
    SIGN_POINT,
    SIGN_COLON,
    SIGN_COMMA,
    SIGN_SEMICOLON,
    SIGN_DOLLAR,
    SIGN_REFERENCE,
    SIGN_DEREFERENCE,
    SIGN_QUOTATION,
    SIGN_SQUOTATION,
    SIGN_AT,
}

impl Into<char> for SIGN_TOKEN {
    fn into(self) -> char {
        match self {
            SIGN_TOKEN::SIGN_BRACKET_CURLY_OPEN => '{',
            SIGN_TOKEN::SIGN_BRACKET_CURLY_CLOSE => '}',
            SIGN_TOKEN::SIGN_BRACKET_ROUND_OPEN => '(',
            SIGN_TOKEN::SIGN_BRACKET_ROUND_CLOSE => ')',
            SIGN_TOKEN::SIGN_BRACKET_SQUARE_OPEN => '[',
            SIGN_TOKEN::SIGN_BRACKET_SQUARE_CLOSE => ']',
            SIGN_TOKEN::SIGN_LESS_THAN => '<',
            SIGN_TOKEN::SIGN_GREATER_THAN => '>',
            SIGN_TOKEN::SIGN_POINT => '.',
            SIGN_TOKEN::SIGN_COLON => ':',
            SIGN_TOKEN::SIGN_COMMA => ',',
            SIGN_TOKEN::SIGN_SEMICOLON => ';',
            SIGN_TOKEN::SIGN_DOLLAR => '$',
            SIGN_TOKEN::SIGN_REFERENCE => '&',
            SIGN_TOKEN::SIGN_DEREFERENCE => '*',
            SIGN_TOKEN::SIGN_QUOTATION => '"',
            SIGN_TOKEN::SIGN_SQUOTATION => '\'',
            SIGN_TOKEN::SIGN_AT => '@',
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Token {
    Text(TEXT_TOKEN),
    Sign(SIGN_TOKEN),
    NL,
    Other(char),
}
impl Into<char> for Token {
    fn into(self) -> char {
        match self {
            Token::Text(v) => v.into(),
            Token::Sign(v) => v.into(),
            Token::NL => '\n',
            Token::Other(v) => v,
        }
    }
}

pub fn tokenize(data: String) -> Vec<Token> {
    let mut is_comment: bool = false;
    let mut res: Vec<Token> = Vec::new();
    for chr in data.chars() {
        if chr == '{' && !is_comment {
            res.push(Token::Sign(SIGN_TOKEN::SIGN_BRACKET_CURLY_OPEN));
        } else if chr == '}' && !is_comment {
            res.push(Token::Sign(SIGN_TOKEN::SIGN_BRACKET_CURLY_CLOSE));
        } else if chr == '(' && !is_comment {
            res.push(Token::Sign(SIGN_TOKEN::SIGN_BRACKET_ROUND_OPEN));
        } else if chr == ')' && !is_comment {
            res.push(Token::Sign(SIGN_TOKEN::SIGN_BRACKET_ROUND_CLOSE));
        } else if chr == '[' && !is_comment {
            res.push(Token::Sign(SIGN_TOKEN::SIGN_BRACKET_SQUARE_OPEN));
        } else if chr == ']' && !is_comment {
            res.push(Token::Sign(SIGN_TOKEN::SIGN_BRACKET_SQUARE_CLOSE));
        } else if chr == '.' && !is_comment {
            res.push(Token::Sign(SIGN_TOKEN::SIGN_POINT));
        } else if chr == ':' && !is_comment {
            res.push(Token::Sign(SIGN_TOKEN::SIGN_COLON));
        } else if chr == ',' && !is_comment {
            res.push(Token::Sign(SIGN_TOKEN::SIGN_COMMA));
        } else if chr == ';' && !is_comment {
            res.push(Token::Sign(SIGN_TOKEN::SIGN_SEMICOLON));
        } else if chr == '$' && !is_comment {
            res.push(Token::Sign(SIGN_TOKEN::SIGN_DOLLAR));
        } else if chr == '&' && !is_comment {
            res.push(Token::Sign(SIGN_TOKEN::SIGN_REFERENCE));
        } else if chr == '*' && !is_comment {
            res.push(Token::Sign(SIGN_TOKEN::SIGN_DEREFERENCE));
        } else if chr == '"' && !is_comment {
            res.push(Token::Sign(SIGN_TOKEN::SIGN_QUOTATION));
        } else if chr == '@' {
            res.push(Token::Sign(SIGN_TOKEN::SIGN_AT));
        }

        //VAR
        else if chr == 'v' && res.last().unwrap() == &Token::NL && !is_comment {
            res.push(Token::Text(TEXT_TOKEN::TEXT_VAR_DECLARATION_1));
        } else if chr == 'a' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_VAR_DECLARATION_1) && !is_comment {
            res.push(Token::Text(TEXT_TOKEN::TEXT_VAR_DECLARATION_2));
        } else if chr == 'r' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_VAR_DECLARATION_2) && !is_comment {
            res.push(Token::Text(TEXT_TOKEN::TEXT_VAR_DECLARATION_3));
        }
        
        //FN 
        else if chr == 'f' && res.last().unwrap() == &Token::NL && !is_comment {
            res.push(Token::Text(TEXT_TOKEN::TEXT_FUNCTION_DECLARATION_1));
        } else if chr == 'n' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_FUNCTION_DECLARATION_1) && !is_comment {
            res.push(Token::Text(TEXT_TOKEN::TEXT_FUNCTION_DECLARATION_2));
        }

        //CL
        else if chr == 'c' && res.last().unwrap() == &Token::NL && !is_comment {
            res.push(Token::Text(TEXT_TOKEN::TEXT_CLASS_DECLARATION_1));
        } else if chr == 'l' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_CLASS_DECLARATION_1) && !is_comment {
            res.push(Token::Text(TEXT_TOKEN::TEXT_CLASS_DECLARATION_2));
        }

        //Comments
        else if chr == '<' && res.last().unwrap() == &Token::NL && !is_comment {
            res.push(Token::Sign(SIGN_TOKEN::SIGN_LESS_THAN));
        } else if chr == '\'' && res.last().unwrap() == &Token::Sign(SIGN_TOKEN::SIGN_LESS_THAN) && !is_comment {
            res.push(Token::Sign(SIGN_TOKEN::SIGN_SQUOTATION));
            is_comment=true;
        } else if chr == '\'' && is_comment {
            res.push(Token::Sign(SIGN_TOKEN::SIGN_SQUOTATION));
        } else if chr == '>' && res.last().unwrap() == &Token::Sign(SIGN_TOKEN::SIGN_SQUOTATION) {
            res.push(Token::Sign(SIGN_TOKEN::SIGN_GREATER_THAN));
            is_comment=false;
        }

        else {
            res.push(Token::NL)
        }
    };
    res.retain(|x|(x)!=&Token::NL);
    let mut output: Vec<Token> = Vec::new();
    for i in 0..res.len() {
        if res[i] == Token::Sign(SIGN_TOKEN::SIGN_LESS_THAN) && res[i+1] == Token::Sign(SIGN_TOKEN::SIGN_SQUOTATION) {

        } else if res[i] == Token::Sign(SIGN_TOKEN::SIGN_SQUOTATION) && res[i-1] == Token::Sign(SIGN_TOKEN::SIGN_LESS_THAN) {

        } else if res[i] == Token::Sign(SIGN_TOKEN::SIGN_SQUOTATION) && res[i+1] == Token::Sign(SIGN_TOKEN::SIGN_GREATER_THAN) {

        } else {
            output.push(res[i]);
        }
    }
    return output;
}
