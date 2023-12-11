extern crate pest;
#[macro_use]
extern crate pest_derive;

use crate::parser::BpParse;

mod parser;

fn main() {
    let file = std::fs::read_to_string("./bp/test.bp").unwrap();
    BpParse::data(&file);

}
