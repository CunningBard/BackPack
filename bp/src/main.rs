extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::io::Write;
use crate::parser::BpParse;
use crate::translation::ToGoCode;

mod parser;
mod translation;

fn main() {
    let file = std::fs::read_to_string("../../temp/test.bp").unwrap();
    let program = BpParse::data(&file);
    
    for construct in program.iter() {
        println!("{:?}", construct);
    }

    // save to file, ./bp/test.go
    let mut file = std::fs::File::create("../../temp/test.go").unwrap();

    let code = program.to_go_code();

    println!("{}", code);

    file.write_all(code.as_bytes()).unwrap();
}
