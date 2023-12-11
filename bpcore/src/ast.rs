

#[derive(Debug)]
pub enum Expression {
    Variable(String),
    String(String),
    Int(i32),
    Float(f32),
    Bool(bool),
    FunctionCall {
        name: String,
        args: Vec<Expression>,
    },
    Unary {
        operator: char,
        right: Box<Expression>,
    },
    Addition {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Subtraction {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Multiplication {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Division {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Equal {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    NotEqual {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    GreaterThan {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    LessThan {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    GreaterThanOrEqual {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    LessThanOrEqual {
        left: Box<Expression>,
        right: Box<Expression>,
    },
}

#[derive(Debug)]
pub enum Statement {
    VariableAssignment {
        name: String,
        var_type: Option<VarType>,
        value: Expression,
    },
    FunctionCall {
        name: String,
        args: Vec<Expression>,
    },
    Return {
        value: Expression,
    },
    Response {
        status_code: Expression,
        body: Expression,
    },
    Conditional {
        conditions: Vec<(Expression, Block)>,
        else_body: Option<Block>,
    },
    WhileLoop {
        condition: Expression,
        body: Block,
    },
}


#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum VarType {
    Int,
    String,
    Float,
    Bool,
    // Struct(String),
}

#[derive(Debug)]
pub struct KeyType {
    pub name: String,
    pub var_type: VarType,
}

#[derive(Debug)]
pub struct Handler {
    pub url_params: Vec<String>,
    pub required_query_params: Vec<KeyType>,
    pub optional_query_params: Vec<KeyType>,
    pub body: Block
}

#[derive(Debug)]
pub struct Method {
    pub method: MethodType,
    pub path: String,
    pub handler: Handler,
}

#[derive(Debug)]
pub enum MethodType {
    Get,
    Post,
    Put,
    Delete,
    Patch,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub args: Vec<KeyType>,
    pub return_type: Option<VarType>,
    pub body: Block,
}