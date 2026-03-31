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
    Not {
        expr: Box<Expression>,
    },
    Negate {
        expr: Box<Expression>,
    },
    List(Vec<Expression>),
    StructLiteral(Vec<(String, Expression)>),
    TypedStructLiteral {
        struct_name: String,
        fields: Vec<(String, Expression)>,
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
    And {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Or {
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
    ForLoop {
        var: String,
        iter: Expression,
        body: Block,
    },
    IfSome {
        binding: String,
        source: Expression,
        some_body: Block,
        none_body: Option<Block>,
    },
}

#[derive(Debug, Clone)]
pub enum Type {
    Int,
    String,
    Float,
    Bool,
    List(Box<Type>),
    Struct(String),
}

impl Type {
    pub fn type_name(&self) -> &'static str {
        match self {
            Type::Int => "int",
            Type::String => "str",
            Type::Float => "float",
            Type::Bool => "bool",
            Type::List(_) => "list",
            Type::Struct(_) => "struct",
        }
    }
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

impl UrlPath {
    pub fn path_str(&self) -> String {
        self.path.iter().map(|p| match p {
            PathType::Path(s) => s.clone(),
            PathType::Variable(s) => format!("{{{}}}", s),
        }).collect::<Vec<_>>().join("/")
    }
}

#[derive(Debug, Clone)]
pub struct Handler {
    pub path: UrlPath,
    pub required_query_params: Vec<KeyType>,
    pub optional_query_params: Vec<KeyType>,
    pub payload_params: Vec<KeyType>,
    pub required_headers: Vec<String>,
    pub expected_headers: Vec<(String, String)>,
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
    Delete,
    Patch,
}

impl MethodType {
    pub fn method_name(&self) -> &'static str {
        match self {
            MethodType::Get => "GET",
            MethodType::Post => "POST",
            MethodType::Put => "PUT",
            MethodType::Delete => "DELETE",
            MethodType::Patch => "PATCH",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub args: Vec<KeyType>,
    pub return_type: Option<VarType>,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<KeyType>,
}

#[derive(Debug, Clone)]
pub struct Import {
    pub path: String,
}

#[derive(Debug, Clone)]
pub struct FromImport {
    pub path: String,
    pub identifier: String,
}

#[derive(Debug, Clone)]
pub enum Construct {
    Function(Function),
    Method(Method),
    Struct(Struct),
    Import(Import),
    FromImport(FromImport),
    // GlobalVariable {
    //     name: String,
    //     var_type: Option<VarType>,
    //     value: Expression,
    // },
}