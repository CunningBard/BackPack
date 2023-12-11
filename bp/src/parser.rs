use pest::iterators::{Pair,};
use pest::Parser;
use bpcore::ast::{Block, Expression, Function, Handler, KeyType, Method, MethodType, Statement, VarType};

#[derive(Parser)]
#[grammar = "bp.pest"]
struct BareParser;

pub struct BpParse;

macro_rules! binary {
    ($left:expr, $right:expr, $op:ident) => {
        Expression::$op {
            left: Box::new($left),
            right: Box::new($right),
        }
    };
}


impl BpParse {
    pub fn typed_arg(pair: Pair<Rule>) -> KeyType {
        let mut pairs = pair.into_inner();
        let arg_name = pairs.next().unwrap();
        let arg_type = Self::var_type(pairs.next().unwrap());
        KeyType {
            name: arg_name.as_str().to_string(),
            var_type: arg_type,
        }
    }
    pub fn var_type(pair: Pair<Rule>) -> VarType {
        match pair.as_str() {
            "string" => VarType::String,
            "int" => VarType::Int,
            "float" => VarType::Float,
            "bool" => VarType::Bool,
            identifier => unimplemented!("Unknown type: {}", identifier)
        }
    }
    pub fn function_call(pair: Pair<Rule>) -> Expression {
        // println!("{:#?}", pair);
        let mut pairs = pair.into_inner();
        let func_name = pairs.next().unwrap();
        let mut args = vec![];

        while let Some(pair) = pairs.next() {
            args.push(Self::expr(pair));
        }

        Expression::FunctionCall {
            name: func_name.as_str().to_string(),
            args,
        }
    }
    pub fn term(pair: Pair<Rule>) -> Expression {
        let mut pairs = pair.into_inner();
        let mut current = pairs.next().unwrap();

        let mut left = match current.as_rule() {
            Rule::string => Expression::String(current.as_str().to_string()),
            Rule::float => Expression::Float(current.as_str().parse().unwrap()),
            Rule::integer => Expression::Int(current.as_str().parse().unwrap()),
            Rule::bool => Expression::Bool(current.as_str().parse().unwrap()),
            Rule::identifier => Expression::Variable(current.as_str().to_string()),
            Rule::function_call => {
                Self::function_call(current)
            },
            Rule::expr => Self::expr(current),
            expr_rule => unimplemented!("{:?}", expr_rule)
        };

        left
    }
    pub fn product(pair: Pair<Rule>) -> Expression {
        let mut pairs = pair.into_inner();
        let current = pairs.next().unwrap();

        let mut left = Self::term(current);

        while let Some(pair) = pairs.next() {
            match pair.as_str() {
                "*" => {
                    left = binary!(left, Self::term(pairs.next().unwrap()), Multiplication);
                }
                "/" => {
                    left = binary!(left, Self::term(pairs.next().unwrap()), Division);
                }
                _ => unimplemented!()
            }
        }

        left
    }

    pub fn sum(pair: Pair<Rule>) -> Expression {
        let mut pairs = pair.into_inner();
        let current = pairs.next().unwrap();

        let mut left = Self::product(current);

        while let Some(pair) = pairs.next() {
            match pair.as_str() {
                "+" => {
                    left = binary!(left, Self::product(pairs.next().unwrap()), Addition);
                }
                "-" => {
                    left = binary!(left, Self::product(pairs.next().unwrap()), Subtraction);
                }
                _ => unimplemented!()
            }
        }

        left
    }
    pub fn expr(pair: Pair<Rule>) -> Expression {
        let mut pairs = pair.into_inner();
        let current = pairs.next().unwrap();


        let mut left = Self::sum(current);

        while let Some(pair) = pairs.next() {
            match pair.as_str() {
                "==" => {
                    left = binary!(left, Self::sum(pairs.next().unwrap()), Equal);
                }
                "!=" => {
                    left = binary!(left, Self::sum(pairs.next().unwrap()), NotEqual);
                }
                "<" => {
                    left = binary!(left, Self::sum(pairs.next().unwrap()), LessThan);
                }
                ">" => {
                    left = binary!(left, Self::sum(pairs.next().unwrap()), GreaterThan);
                }
                "<=" => {
                    left = binary!(left, Self::sum(pairs.next().unwrap()), LessThanOrEqual);
                }
                ">=" => {
                    left = binary!(left, Self::sum(pairs.next().unwrap()), GreaterThanOrEqual);
                }
                _ => unimplemented!()
            }
        }

        left
    }
    pub fn variable_assignment(pair: Pair<Rule>) -> Statement {
        let mut pairs = pair.into_inner();
        let variable_name = pairs.next().unwrap();

        let mut current = pairs.next().unwrap();

        let var_type = if current.as_rule() == Rule::identifier {
            let var_type = Self::var_type(current);
            current = pairs.next().unwrap();
            Some(var_type)
        } else {
            None
        };

        let value = Self::expr(current);

        Statement::VariableAssignment {
            name: variable_name.as_str().to_string(),
            var_type,
            value,
        }
    }
    pub fn statement(pair: Pair<Rule>) -> Statement {
        match pair.as_rule() {
            Rule::variable_assignment => {
                Self::variable_assignment(pair)
            }
            Rule::response => {
                Self::respond(pair)
            }
            Rule::return_statement => {
                Self::return_statement(pair)
            }
            thing => unimplemented!("Unknown statement: {:?}", thing)
        }
    }

    pub fn block(mut pair: Pair<Rule>) -> Block {
        let pairs = pair.into_inner();

        let mut statements = vec![];

        for pair in pairs {
            statements.push(
                Self::statement(pair)
            )
        }

        Block { statements }
    }
    pub fn function(pair: Pair<Rule>) -> Function {
        // function = {
        //     "func" ~ identifier ~
        //     "("
        //     ~ (typed_arg ~ ",")*
        //     ~ (typed_arg)? ~
        //     ")"
        //     ~ ("->" ~ identifier)? ~ block
        // }

        let mut pairs = pair.into_inner();

        let func_name = pairs.next().unwrap();

        let mut node = pairs.next().unwrap();
        let mut args = vec![];

        while node.as_rule() == Rule::typed_arg {
            args.push(Self::typed_arg(node));
            node = pairs.next().unwrap();
        }

        let return_type = if node.as_rule() == Rule::identifier {
            let var_type = Self::var_type(node);
            node = pairs.next().unwrap();
            Some(var_type)
        } else {
            None
        };

        let block = Self::block(node);

        Function {
            name: func_name.as_str().to_string(),
            args,
            return_type,
            body: block,
        }
    }
    pub fn method_type(pair: Pair<Rule>) -> MethodType {
        match pair.as_str() {
            "get" => MethodType::Get,
            "post" => MethodType::Post,
            "put" => MethodType::Put,
            "delete" => MethodType::Delete,
            "patch" => MethodType::Patch,
            _ => unimplemented!()
        }
    }
    pub fn method(pair: Pair<Rule>) -> Method {
        // method = {
        //     method_type ~ identifier ~
        //     "[" ~
        //     ( path_type | optional_query | typed_arg ~ ",")* ~
        //     ( path_type | optional_query | typed_arg )? ~
        //     "]" ~
        //     "("
        //     ~ (typed_arg ~ ",")*
        //     ~ (typed_arg)? ~
        //     ")"
        //     ~ block
        // }

        let mut pairs = pair.into_inner();

        let method_type = Self::method_type(pairs.next().unwrap());
        let endpoint = pairs.next().unwrap().as_str();

        //     "[" ~
        //     ( path_type | optional_query | typed_arg ~ ",")* ~
        //     ( path_type | optional_query | typed_arg )? ~
        //     "]" ~

        let mut optionals = vec![];
        let mut required = vec![];
        let mut path_args = vec![];

        let mut node = pairs.next().unwrap();
        while node.as_rule() == Rule::path_type || node.as_rule() == Rule::optional_query || node.as_rule() == Rule::typed_arg {
            match node.as_rule() {
                Rule::path_type => {
                    let path = node.as_str().trim_start_matches("/");
                    path_args.push(path.to_string());
                }
                Rule::optional_query => {
                    let mut pairs = node.into_inner();
                    let arg = Self::typed_arg(pairs.next().unwrap());
                    optionals.push(arg);
                }
                Rule::typed_arg => {
                    required.push(Self::typed_arg(node));
                }
                _ => unimplemented!()
            }
            node = pairs.next().unwrap();
        }

        //     "("
        //     ~ (typed_arg ~ ",")*
        //     ~ (typed_arg)? ~
        //     ")"

        let mut args = vec![];
        while node.as_rule() == Rule::typed_arg {
            args.push(Self::typed_arg(node));
            node = pairs.next().unwrap();
        }

        let block = Self::block(node);

        let handler = Handler {
            url_params: path_args,
            required_query_params: required,
            optional_query_params: optionals,
            body: block,
        };

        Method {
            method: method_type,
            path: endpoint.to_string(),
            handler,
        }
    }
    pub fn respond(pair: Pair<Rule>) -> Statement {
        let mut pairs = pair.into_inner();
        let status_code = Self::expr(pairs.next().unwrap());
        let body = Self::expr(pairs.next().unwrap());
        Statement::Response {
            status_code,
            body
        }
    }

    pub fn return_statement(pair: Pair<Rule>) -> Statement {
        let mut pairs = pair.into_inner();
        let value = Self::expr(pairs.next().unwrap());
        Statement::Return {
            value
        }
    }
    pub fn data(data: &str){
        let pairs = BareParser::parse(Rule::program, data)
            .unwrap_or_else(|e| panic!("{}", e));

        for pair in pairs {
            match pair.as_rule() {
                Rule::function => {
                    let func = Self::function(pair);
                    println!("{:#?}", func);
                }
                Rule::method => {
                    // println!("{:#?}", pair);
                    let method = Self::method(pair);
                    println!("{:#?}", method);
                }
                Rule::EOI => {
                    break;
                }
                _ => unimplemented!()
            }
        }
    }
}