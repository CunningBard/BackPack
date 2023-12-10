

enum Expression {
    Variable(String),
    String(String),
    Integer(i32),
    Float(f32),
    Boolean(bool),
    FunctionCall {
        name: String,
        args: Vec<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: char,
        right: Box<Expression>,
    },
    Unary {
        operator: char,
        right: Box<Expression>,
    },
}

pub enum Statement {
    VariableAssignment {
        name: String,
        value: Expression,
    },
    FunctionCall {
        name: String,
        args: Vec<Expression>,
    },
    Return(Expression),
    Response(Expression, Expression),
    Conditional {
        conditions: Vec<(Expression, Block)>,
        else_body: Option<Block>,
    },
    WhileLoop {
        condition: Expression,
        body: Block,
    },
}


pub struct Block {
    pub statements: Vec<Statement>,
}

pub enum VarType {
    Int,
    String,
    Float,
    Bool,
}

pub struct KeyType {
    pub name: String,
    pub var_type: VarType,
}

pub struct Handler {
    pub url_params: Vec<String>,
    pub required_query_params: Vec<KeyType>,
    pub optional_query_params: Vec<KeyType>,
    pub body: Block
}

pub struct Method {
    pub method: MethodType,
    pub path: String,
    pub handler: Handler,
}

pub enum MethodType {
    Get,
    Post,
    // Put,
    // Delete,
    // Patch,
}