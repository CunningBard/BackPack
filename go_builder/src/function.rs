use crate::data_type::DataType;
use crate::Statement;

pub struct Function {
    pub name: String,
    pub args: Vec<(String, DataType)>,
    pub return_type: Option<DataType>,
    pub body: Vec<Statement>,
}