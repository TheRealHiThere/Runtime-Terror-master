use crate::tokenizer::*;


#[derive(Debug, Clone, Default)]
pub struct Arg {
    pub name: String,
    pub atype: String
}


#[derive(Debug, Clone, Default)]
pub struct Function {
    pub name: String,
    pub args: Vec<Arg>,
    pub body: Vec<Token>,
}
impl Function {
    pub fn new(name: String, args: Vec<Arg>, body: Vec<Token>) -> Function { Function { name, args, body } }
    pub fn constructor(name: &String) -> Function { Function::new(name.to_owned(), vec![], vec![]) }
}


#[derive(Debug, Clone, Default)]
pub struct Class {
    pub name: String,
    pub constructor: Function,
    pub properties: Vec<Arg>,
    pub methods: Vec<Function>,
}
impl Class {
    pub fn new(name: &String) -> Self {
        Class { name: String::clone(&name), constructor: Function::constructor(&name), properties: Vec::new() ,methods: Vec::new() }
    }
    pub fn push_method(&mut self, method: Function) {
        self.methods.push(method);
    }
}