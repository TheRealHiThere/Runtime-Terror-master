mod tokenizer;
mod lexer;
use std::{
    io::{self,prelude::*},
    fs::File
};

fn main() -> io::Result<()> {
    let mut file = File::open("test.rt")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let mut outputfile = File::create("output.txt")?;
    let tokens = tokenizer::tokenize(data);
    outputfile.write_all(format!("{:#?}\n\n",&tokens).as_bytes())?;
    let res = lexer::lex(tokens);
    outputfile.write_all(format!("{:#?}",res).as_bytes())?;

    Ok(())
}