pub mod function;
pub mod variable;
pub mod data_type;
pub mod structs;
pub mod expr;

pub enum Construct {
    Function(function::Function),
    Variable(variable::Variable),
    // Struct(structs::Struct),
}

pub enum Statement {
    
}