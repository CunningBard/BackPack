extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::io::Write;
use crate::parser::BpParse;
use crate::translation::ToGoCode;
use crate::type_check::type_check_program;

mod parser;
mod translation;
mod type_check;

fn main() {
    let file = std::fs::read_to_string("temp/test.bp").unwrap();
    let mut program = BpParse::data(&file);
    
    if let Err(errors) = type_check_program(&mut program) {
        eprintln!("Type checking failed:");
        for error in errors {
            eprintln!("  {}", error);
        }
        std::process::exit(1);
    }
    
    for construct in program.iter() {
        match construct {
            bpcore::ast::Construct::Function(f) => {
                let args: Vec<_> = f.args.iter().map(|a| format!("{}: {}", a.name, a.var_type.var_type.type_name())).collect();
                println!("[function] {}({})", f.name, args.join(", "));
            }
            bpcore::ast::Construct::Method(m) => {
                println!("[route]    {} /{} -> {}Handler", m.method.method_name(), m.handler.path.path_str(), m.path);
            }
            bpcore::ast::Construct::Struct(s) => {
                println!("[struct]   {} ({} fields)", s.name, s.fields.len());
            }
            bpcore::ast::Construct::Import(i) => {
                println!("[import]   {}", i.path);
            }
            bpcore::ast::Construct::FromImport(i) => {
                println!("[import]   from {} import {}", i.path, i.identifier);
            }
        }
    }
    
    println!("✓ Type checking passed");

    // save to file, ./bp/test.go
    let mut file = std::fs::File::create("temp/test.go").unwrap();

    let code = program.to_go_code();

    println!("{}", code);

    file.write_all(code.as_bytes()).unwrap();
}
