use std::fmt::Display;

pub type Block = Vec<Statement>;
pub type Program = Vec<Construct>;

#[derive(Debug, Clone)]
pub enum Identifier {
    Single(String),
    DotIdentifier(Vec<String>), // variables with dots in them
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Identifier::Single(s) => s.clone(),
            Identifier::DotIdentifier(v) => v.join("."),
        };
        write!(f, "{}", str)
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Variable(Identifier),
    String(String),
    Int(i32),
    Float(f32),
    Bool(bool),
    FunctionCall {
        func_name: Identifier,
        args: Vec<Expression>,
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

#[derive(Debug, Clone)]
pub enum Statement {
    VariableAssignment {
        name: String,
        var_type: Option<VarType>,
        value: Expression,
    },
    VariableReassignment {
        variable_name: Identifier,
        value: Expression,
    },
    FunctionCall {
        name: Identifier,
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

#[derive(Debug, Clone)]
pub enum Type {
    Int,
    String,
    Float,
    Bool,
    // Struct(String),
}

#[derive(Debug, Clone)]
pub struct VarType {
    pub pointer_count: usize,
    pub var_type: Type,
}

#[derive(Debug, Clone)]
pub struct KeyType {
    pub name: String,
    pub var_type: VarType,
}


#[derive(Debug, Clone)]
pub enum PathType {
    Variable(String),
    Path(String),
}


#[derive(Debug, Clone)]
pub struct UrlPath {
    pub path: Vec<PathType>
}

#[derive(Debug, Clone)]
pub struct Handler {
    pub path: UrlPath,
    pub required_query_params: Vec<KeyType>,
    pub optional_query_params: Vec<KeyType>,
    pub body: Block
}

#[derive(Debug, Clone)]
pub struct Method {
    pub method: MethodType,
    pub path: String,
    pub handler: Handler,
}

#[derive(Debug, Clone)]
pub enum MethodType {
    Get,
    Post,
    Put,
    // Delete,
    // Patch,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub args: Vec<KeyType>,
    pub return_type: Option<VarType>,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub enum Construct {
    Function(Function),
    Method(Method),
    // GlobalVariable {
    //     name: String,
    //     var_type: Option<VarType>,
    //     value: Expression,
    // },
}