use crate::data_type::DataType;
use crate::expr::Expr;

pub struct Variable {
    pub name: String,
    pub var_type: Option<DataType>,
    pub value: Option<Expr>,
}