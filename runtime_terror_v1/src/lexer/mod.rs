#![allow(unused)]


use crate::tokenizer::*;
mod standard;
use standard::*;


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Lexes {
    Function,
    Method,
    Class,
    Void
}

#[derive(Debug, Clone)]
pub enum Lexed {
    Function(Function),
    Class(Class),
}

pub(crate) fn lex(tokes: Vec<Token>) -> Vec<Lexed> {
    let mut lexes: Vec<Lexed> = Vec::new();
    let mut currently_constructing: Lexes = Lexes::Void;
    let mut currently_data: String = String::new();
    for token in tokes {
        match token {
            Token::Sign(SIGN_TOKEN::SIGN_SEMICOLON) => {
                match currently_constructing {
                    // Lexes::Function => {
                    //     // let function = Function::new(currently_data.clone(), vec![], vec![]);
                    //     // lexes.push(Lexed::Function(function));
                    //     // currently_data = String::new();
                    //     // currently_constructing = Lexes::Void;
                    // },
                    Lexes::Method => {
                        currently_constructing = Lexes::Void;
                        // let method = Function::new(currently_data.clone(), vec![], vec![]);
                        // lexes.push(Lexed::Function(method));
                        // currently_data = String::new();
                    },
                    // Lexes::Class => {
                    //     let class = Class::new(&currently_data);
                    //     lexes.push(Lexed::Class(class));
                    //     currently_data = String::new();
                    // },
                    // Lexes::Void => {}
                    _ => {
                        currently_constructing = Lexes::Void;
                    }
                }
            },
            Token::Text(TEXT_TOKEN::TEXT_DEC(v)) => {
                currently_data.push(v)
            },
            Token::Sign(SIGN_TOKEN::SIGN_BRACKET_SQUARE_OPEN) => {
                match currently_constructing {
                    Lexes::Class => {
                        let currently_class = Class::new(&currently_data);
                        currently_data = String::new();
                        currently_constructing = Lexes::Method;
                        lexes.push(Lexed::Class(currently_class));
                    },
                    Lexes::Method => {
                        let mut klazz = match lexes.pop().unwrap() {
                            Lexed::Class(klazz) => klazz,
                            _ => panic!("Expected class")
                        };
                        klazz.push_method(Function::constructor(&currently_data));
                        lexes.push(Lexed::Class(klazz));
                        currently_data = String::new();
                        currently_constructing = Lexes::Method;
                    },
                    Lexes::Function => {
                        lexes.push(Lexed::Function(Function::constructor(&currently_data)));
                        currently_data = String::new();
                        currently_constructing = Lexes::Void;
                    },
                    _ => {}
                }
            },

            Token::Text(TEXT_TOKEN::TEXT_CLASS_DECLARATION_2) => {
                currently_constructing = Lexes::Class;
            },
            Token::Text(TEXT_TOKEN::TEXT_FUNCTION_DECLARATION_2) => {
                if currently_constructing != Lexes::Method {
                    currently_constructing = Lexes::Function;
                } else {
                    currently_constructing = Lexes::Method;
                }
            },

            Token::Sign(SIGN_TOKEN::SIGN_BRACKET_CURLY_CLOSE) => {
                match currently_constructing {
                    Lexes::Method => {
                        currently_constructing = Lexes::Method;
                    },
                    _ => {
                        currently_constructing = Lexes::Void;
                    }
                }
            },
            _ => {}
        }
    };
    lexes
}
