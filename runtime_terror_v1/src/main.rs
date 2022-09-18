mod tokenizer;
use std::{
    io::{self,prelude::*},
    fs::File
};

fn main() -> io::Result<()> {
    let mut file = File::open("test.rt")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;


    let res = tokenizer::tokenize(data);
    println!("{:?}",res);



    Ok(())
}