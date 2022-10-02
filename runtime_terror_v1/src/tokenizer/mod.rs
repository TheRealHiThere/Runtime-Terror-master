mod consts;
pub use consts::*;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Token {
    Text(TEXT_TOKEN),
    Sign(SIGN_TOKEN),
    SPACE,
    NL
}
impl Into<char> for Token {
    fn into(self) -> char {
        match self {
            Token::Text(v) => v.into(),
            Token::Sign(v) => v.into(),
            Token::SPACE => ' ',
            Token::NL => '\n'
        }
    }
}

pub fn tokenize(data: String) -> Vec<Token> {
    let mut is_args: bool = false;
    let mut is_comment: bool = false;
    let mut some_dec: bool = false;
    let mut res: Vec<Token> = Vec::new();
    res.push(Token::NL);
    for chr in data.chars() {
        if chr == '{' && !is_comment && !is_args{
            res.push(Token::Sign(SIGN_TOKEN::SIGN_BRACKET_CURLY_OPEN));
        } else if chr == '}' && !is_comment && !is_args{
            res.push(Token::Sign(SIGN_TOKEN::SIGN_BRACKET_CURLY_CLOSE));
        } else if chr == '(' && !is_comment {
            if some_dec { some_dec = false; }
            is_args = true;
            res.push(Token::Sign(SIGN_TOKEN::SIGN_BRACKET_ROUND_OPEN));
        } else if chr == ')' && !is_comment {
            res.push(Token::Sign(SIGN_TOKEN::SIGN_BRACKET_ROUND_CLOSE));
            is_args = false;
        } else if chr == '[' && !is_comment && !is_args {
            if some_dec { some_dec = false; }
            res.push(Token::Sign(SIGN_TOKEN::SIGN_BRACKET_SQUARE_OPEN));
        } else if chr == ']' && !is_comment && !is_args{
            res.push(Token::Sign(SIGN_TOKEN::SIGN_BRACKET_SQUARE_CLOSE));
        } else if chr == '.' && !is_comment && !is_args{
            res.push(Token::Sign(SIGN_TOKEN::SIGN_POINT));
        } else if chr == ':' && !is_comment && !is_args{
            res.push(Token::Sign(SIGN_TOKEN::SIGN_COLON));
        } else if chr == ',' && !is_comment && !is_args {
            res.push(Token::Sign(SIGN_TOKEN::SIGN_COMMA));
        } else if chr == ';' && !is_comment && !is_args {
            res.push(Token::Sign(SIGN_TOKEN::SIGN_SEMICOLON));
        } else if chr == '$' && !is_comment && !is_args {
            res.push(Token::Sign(SIGN_TOKEN::SIGN_DOLLAR));
        } else if chr == '&' && !is_comment && !is_args {
            res.push(Token::Sign(SIGN_TOKEN::SIGN_REFERENCE));
        } else if chr == '*' && !is_comment && !is_args {
            res.push(Token::Sign(SIGN_TOKEN::SIGN_DEREFERENCE));
        } else if chr == '"' && !is_comment && !is_args {
            res.push(Token::Sign(SIGN_TOKEN::SIGN_QUOTATION));
        } else if chr == '@' && !is_comment && !is_args {
            res.push(Token::Sign(SIGN_TOKEN::SIGN_AT));
        } else if chr == '<' && !is_comment && !is_args {
            res.push(Token::Sign(SIGN_TOKEN::SIGN_LESS_THAN));
        } else if chr == '>' && !is_comment && !is_args {
            res.push(Token::Sign(SIGN_TOKEN::SIGN_GREATER_THAN));
        }

        //VAR
        else if chr == 'v' && res.last().unwrap() == &Token::NL && !is_comment && !is_args {
            res.push(Token::Text(TEXT_TOKEN::TEXT_VAR_DECLARATION_1));
        } else if chr == 'a' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_VAR_DECLARATION_1) && !is_comment && !is_args {
            res.push(Token::Text(TEXT_TOKEN::TEXT_VAR_DECLARATION_2));
        } else if chr == 'r' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_VAR_DECLARATION_2) && !is_comment && !is_args {
            res.push(Token::Text(TEXT_TOKEN::TEXT_VAR_DECLARATION_3));
        }
        
        //FN 
        else if chr == 'f' && res.last().unwrap() == &Token::NL && !is_comment && !is_args {
            res.push(Token::Text(TEXT_TOKEN::TEXT_FUNCTION_DECLARATION_1));
        } else if chr == 'n' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_FUNCTION_DECLARATION_1) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_FUNCTION_DECLARATION_2));
        }

        //RETURN
        else if chr == 'r' && res.last().unwrap() == &Token::NL && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_RETURN_DECLARATION_1));
        } else if chr == 'e' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_RETURN_DECLARATION_1) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_RETURN_DECLARATION_2));
        } else if chr == 't' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_RETURN_DECLARATION_2) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_RETURN_DECLARATION_3));
        } else if chr == 'u' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_RETURN_DECLARATION_3) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_RETURN_DECLARATION_4));
        } else if chr == 'r' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_RETURN_DECLARATION_4) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_RETURN_DECLARATION_5));
        } else if chr == 'n' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_RETURN_DECLARATION_5) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_RETURN_DECLARATION_6));
        }

        //CL
        else if chr == 'c' && res.last().unwrap() == &Token::NL && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_CLASS_DECLARATION_1));
        } else if chr == 'l' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_CLASS_DECLARATION_1) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_CLASS_DECLARATION_2));
        }

        //CONST
        else if chr == 'o' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_CLASS_DECLARATION_1) && !is_comment && !is_args{
            let _ = res.pop();
            res.push(Token::Text(TEXT_TOKEN::TEXT_CONST_DECLARATION_1));
            res.push(Token::Text(TEXT_TOKEN::TEXT_CONST_DECLARATION_2));
        } else if chr == 'n' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_CONST_DECLARATION_2) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_CONST_DECLARATION_3));
        } else if chr == 's' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_CONST_DECLARATION_3) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_CONST_DECLARATION_4));
        } else if chr == 't' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_CONST_DECLARATION_4) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_CONST_DECLARATION_5));
        }

        //IMPORTS
        else if chr == 'i' && res.last().unwrap() == &Token::NL && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_IMPORT_DECLARATION_1));
        } else if chr == 'm' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_IMPORT_DECLARATION_1) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_IMPORT_DECLARATION_2));
        } else if chr == 'p' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_IMPORT_DECLARATION_2) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_IMPORT_DECLARATION_3));
        } else if chr == 'o' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_IMPORT_DECLARATION_3) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_IMPORT_DECLARATION_4));
        } else if chr == 'r' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_IMPORT_DECLARATION_4) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_IMPORT_DECLARATION_5));
        } else if chr == 't' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_IMPORT_DECLARATION_5) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_IMPORT_DECLARATION_6));
        }

        //DERIVE
        else if chr == 'd' && res.last().unwrap() == &Token::NL && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_DERIVE_DECLARATION_1));
        } else if chr == 'e' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_DERIVE_DECLARATION_1) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_DERIVE_DECLARATION_2));
        } else if chr == 'r' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_DERIVE_DECLARATION_2) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_DERIVE_DECLARATION_3));
        } else if chr == 'i' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_DERIVE_DECLARATION_3) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_DERIVE_DECLARATION_4));
        } else if chr == 'v' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_DERIVE_DECLARATION_4) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_DERIVE_DECLARATION_5));
        } else if chr == 'e' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_DERIVE_DECLARATION_5) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_DERIVE_DECLARATION_6));
        }

        //SWITCH
        else if chr == 's' && res.last().unwrap() == &Token::NL && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_SWITCH_DECLARATION_1));
        } else if chr == 'w' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_SWITCH_DECLARATION_1) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_SWITCH_DECLARATION_2));
        } else if chr == 'i' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_SWITCH_DECLARATION_2) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_SWITCH_DECLARATION_3));
        } else if chr == 't' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_SWITCH_DECLARATION_3) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_SWITCH_DECLARATION_4));
        } else if chr == 'c' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_SWITCH_DECLARATION_4) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_SWITCH_DECLARATION_5));
        } else if chr == 'h' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_SWITCH_DECLARATION_5) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_SWITCH_DECLARATION_6));
        }

        //STATIC
        else if chr == 't' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_STATIC_DECLARATION_1) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_STATIC_DECLARATION_2));
        } else if chr == 'a' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_STATIC_DECLARATION_2) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_STATIC_DECLARATION_3));
        } else if chr == 't' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_STATIC_DECLARATION_3) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_STATIC_DECLARATION_4));
        } else if chr == 'i' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_STATIC_DECLARATION_4) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_STATIC_DECLARATION_5));
        } else if chr == 'c' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_STATIC_DECLARATION_5) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_STATIC_DECLARATION_6));
        }

        //NEW
        else if chr == 'n' && res.last().unwrap() == &Token::NL && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_NEW_DECLARATION_1));
        } else if chr == 'e' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_NEW_DECLARATION_1) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_NEW_DECLARATION_2));
        } else if chr == 'w' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_NEW_DECLARATION_2) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_NEW_DECLARATION_3));
        }

        //COMMENTS
        else if chr == '\''  && !is_comment && !is_args{
            res.push(Token::Sign(SIGN_TOKEN::SIGN_SQUOTATION));
            is_args = false;
            is_comment=true;
        } else if chr == '\'' && is_comment && !is_args{
            res.push(Token::Sign(SIGN_TOKEN::SIGN_SQUOTATION));
            is_comment=false;
            is_args = false;
        }

        // ENUM
        else if chr == 'e' && res.last().unwrap() == &Token::NL && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_ENUM_DECLARATION_1));
        } else if chr == 'n' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_ENUM_DECLARATION_1) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_ENUM_DECLARATION_2));
        } else if chr == 'u' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_ENUM_DECLARATION_2) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_ENUM_DECLARATION_3));
        } else if chr == 'm' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_ENUM_DECLARATION_3) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_ENUM_DECLARATION_4));
        }

        //MUT
        else if chr == 'm' && res.last().unwrap() == &Token::NL && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_MUT_DECLARATION_1));
        } else if chr == 'u' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_MUT_DECLARATION_1) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_MUT_DECLARATION_2));
        } else if chr == 't' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_MUT_DECLARATION_2) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_MUT_DECLARATION_3));
        }

        //TYPE
        else if chr == 't' && res.last().unwrap() == &Token::NL && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_TYPE_DECLARATION_1));
        } else if chr == 'y' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_TYPE_DECLARATION_1) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_TYPE_DECLARATION_2));
        } else if chr == 'p' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_TYPE_DECLARATION_2) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_TYPE_DECLARATION_3));
        } else if chr == 'e' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_TYPE_DECLARATION_3) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_TYPE_DECLARATION_4));
        }

        //UNSAFE
        else if chr == 'u' && res.last().unwrap() == &Token::NL && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_UNSAFE_DECLARATION_1));
        } else if chr == 'n' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_UNSAFE_DECLARATION_1) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_UNSAFE_DECLARATION_2));
        } else if chr == 's' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_UNSAFE_DECLARATION_2) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_UNSAFE_DECLARATION_3));
        } else if chr == 'a' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_UNSAFE_DECLARATION_3) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_UNSAFE_DECLARATION_4));
        } else if chr == 'f' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_UNSAFE_DECLARATION_4) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_UNSAFE_DECLARATION_5));
        } else if chr == 'e' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_UNSAFE_DECLARATION_5) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_UNSAFE_DECLARATION_6));
        }

        //PUB
        else if chr == 'p' && res.last().unwrap() == &Token::NL && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_PUB_DECLARATION_1));
        } else if chr == 'u' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_PUB_DECLARATION_1) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_PUB_DECLARATION_2));
        } else if chr == 'b' && res.last().unwrap() == &Token::Text(TEXT_TOKEN::TEXT_PUB_DECLARATION_2) && !is_comment && !is_args{
            res.push(Token::Text(TEXT_TOKEN::TEXT_PUB_DECLARATION_3));
        }

        else if chr == ' ' && !is_comment && res.last().unwrap() != &Token::NL {
            res.push(Token::SPACE);
            some_dec=true;
        } else if chr == '\n' || chr == '\r' {
            res.push(Token::NL);
        } else if !is_comment && some_dec && !is_args {
            res.push(Token::Text(TEXT_TOKEN::TEXT_DEC(chr)));
        } else if !is_comment && is_args && !some_dec {
            res.push(Token::Text(TEXT_TOKEN::TEXT_OTHER(chr)));
        }
    };

    res.retain(|x|(x)!=&Token::NL);
    res.retain(|x|(x)!=&Token::Sign(SIGN_TOKEN::SIGN_SQUOTATION));
    return res;
}