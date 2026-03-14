extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::io::Write;
use crate::parser::BpParse;
use crate::translation::ToGoCode;

mod parser;
mod translation;

fn main() {
    let file = std::fs::read_to_string("temp/test.bp").unwrap();
    let program = BpParse::data(&file);
    
    for construct in program.iter() {
        match construct {
            bpcore::ast::Construct::Function(f) => {
                let args: Vec<_> = f.args.iter().map(|a| format!("{}: {}", a.name, a.var_type.var_type.type_name())).collect();
                println!("[function] {}({})", f.name, args.join(", "));
            }
            bpcore::ast::Construct::Method(m) => {
                println!("[route]    {} /{} -> {}Handler", m.method.method_name(), m.handler.path.path_str(), m.path);
            }
        }
    }

    // save to file, ./bp/test.go
    let mut file = std::fs::File::create("temp/test.go").unwrap();

    let code = program.to_go_code();

    println!("{}", code);

    file.write_all(code.as_bytes()).unwrap();
}
