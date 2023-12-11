extern crate pest;
#[macro_use]
extern crate pest_derive;

use crate::parser::BpParse;

mod parser;

fn main() {

    let data = r#"
    get hello[/name, age: int](){
        respond 200, format(
            "Hello, {}! You are {} years old.",
            name,
            age
        );
    }
    "#;
    BpParse::data(data);

}
