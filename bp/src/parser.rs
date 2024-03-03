use pest::iterators::{Pair};
use pest::Parser;
use bpcore::ast::{Block, Construct, Expression, Function, Handler, Identifier, KeyType, Method, MethodType, PathType, Statement, Type, UrlPath, VarType};

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

macro_rules! v_type {
    ($rtype:ident, $current:expr, $var_type:ident, $pairs:ident) => {
        if $current.as_rule() == Rule::$rtype {
            let $var_type = Self::var_type($current);
            $current = $pairs.next().unwrap();
            Some($var_type)
        } else {
            None
        }
    };
}


impl BpParse {
    pub fn identifier_with_dots(pair: Pair<Rule>) -> Identifier {
        let mut pairs = pair.into_inner();
        let mut identifiers = vec![];
        while let Some(pair) = pairs.next() {
            identifiers.push(pair.as_str().to_string());
        }

        Identifier::DotIdentifier(identifiers)

    }
    pub fn usable_identifier(pair: Pair<Rule>) -> Identifier {
        let mut pairs = pair.into_inner();
        match pairs.next() {
            Some(pair) => {
                match pair.as_rule() {
                    Rule::identifier => {
                        Identifier::Single(pair.as_str().to_string())
                    }
                    Rule::identifier_with_dots => {
                        Self::identifier_with_dots(pair)
                    }
                    _ => unimplemented!()
                }
            }
            None => {
                unreachable!()
            }
        }
    }
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
        let var_type = match pair.as_str() {
            "string" => Type::String,
            "int" => Type::Int,
            "float" => Type::Float,
            "bool" => Type::Bool,
            identifier => unimplemented!("Unknown type: {}", identifier)
        };

        let pointer_count = 0;

        VarType {
            pointer_count,
            var_type
        }

    }
    pub fn function_call(pair: Pair<Rule>) -> Expression {
        // println!("{:#?}", pair);
        let mut pairs = pair.into_inner();
        let func_name = Self::usable_identifier(pairs.next().unwrap());
        let mut args = vec![];

        while let Some(pair) = pairs.next() {
            args.push(Self::expr(pair));
        }

        Expression::FunctionCall {
            func_name,
            args,
        }
    }
    pub fn term(pair: Pair<Rule>) -> Expression {
        let mut pairs = pair.into_inner();
        let current = pairs.next().unwrap();

        let left = match current.as_rule() {
            Rule::string => Expression::String(current.as_str().to_string()),
            Rule::float => Expression::Float(current.as_str().parse().unwrap()),
            Rule::integer => Expression::Int(current.as_str().parse().unwrap()),
            Rule::bool => Expression::Bool(current.as_str().parse().unwrap()),
            Rule::usable_identifier => Expression::Variable(Self::usable_identifier(current)),
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

        let var_type = v_type!(identifier, current, var_type, pairs);

        let value = Self::expr(current);

        Statement::VariableAssignment {
            name: variable_name.as_str().to_string(),
            var_type,
            value,
        }
    }

    pub fn conditional(pair: Pair<Rule>) -> Statement {
        let mut pairs = pair.into_inner();
        let mut current = pairs.next().unwrap();

        let mut conditions = vec![];

        while current.as_rule() == Rule::expr {
            let condition = Self::expr(current);
            let body = Self::block(pairs.next().unwrap());
            conditions.push((condition, body));
            match pairs.next() {
                Some(pair) => {
                    current = pair;
                }
                None => {
                    return Statement::Conditional {
                        conditions,
                        else_body: None,
                    }
                }
            }
        }

        let else_body = if current.as_rule() == Rule::block {
            Some(Self::block(current))
        } else {
            None
        };

        Statement::Conditional {
            conditions,
            else_body,
        }
    }
    pub fn while_loop(pair: Pair<Rule>) -> Statement {
        let mut pairs = pair.into_inner();
        let condition = Self::expr(pairs.next().unwrap());
        let body = Self::block(pairs.next().unwrap());
        Statement::WhileLoop {
            condition,
            body,
        }
    }
    pub fn variable_reassignment(pair: Pair<Rule>) -> Statement {
        let mut pairs = pair.into_inner();
        let variable_name = Self::usable_identifier(pairs.next().unwrap());
        let value = Self::expr(pairs.next().unwrap());
        Statement::VariableReassignment {
            variable_name,
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
            Rule::function_call => {
                let fn_call = Self::function_call(pair);

                if let Expression::FunctionCall { func_name: name, args } = fn_call {
                    Statement::FunctionCall {
                        name,
                        args,
                    }
                } else {
                    unreachable!()
                }
            }
            Rule::conditional => {
                Self::conditional(pair)
            }
            Rule::while_loop => {
                Self::while_loop(pair)
            }
            Rule::variable_reassignment => {
                Self::variable_reassignment(pair)
            }
            thing => unimplemented!("Unknown statement: {:?}", thing)
        }
    }

    pub fn block(pair: Pair<Rule>) -> Block {
        let pairs = pair.into_inner();

        let mut statements = vec![];

        for pair in pairs {
            statements.push(
                Self::statement(pair)
            )
        }

        statements
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

        let return_type = v_type!(identifier, node, return_type, pairs);

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
            "GET" => MethodType::Get,
            "POST" => MethodType::Post,
            "PUT" => MethodType::Put,
            // "DELETE" => MethodType::Delete,
            // "PATCH" => MethodType::Patch,
            _ => unimplemented!()
        }
    }

    pub fn path_specifier(pair: Pair<Rule>) -> UrlPath {
        let mut pairs = pair.into_inner();
        let mut path = vec![];

        while let Some(pair) = pairs.next() {
            match pair.as_rule() {
                Rule::identifier => {
                    path.push(PathType::Path(pair.as_str().to_string()));
                }
                Rule::enclosed_path => {
                    let mut inner_pairs = pair.into_inner();
                    let inner_path = inner_pairs.next().unwrap().as_str();
                    
                    path.push(PathType::Variable(inner_path.to_string()));
                }
                _ => unimplemented!()
            }
        }

        UrlPath {
            path
        }
    }
    /// Pest Syntax
    /// ```pest
    /// query = {
    ///     identifier ~ ":" ~ identifier
    /// }
    /// 
    /// optional_query = {
    ///     query ~ "?"
    /// }
    /// 
    /// query_specifier = {
    ///     "QUERY" ~ (optional_query | query)
    /// }
    /// ```
    pub fn query_specifier(pair: Pair<Rule>) -> (KeyType, bool) {
        let mut pairs = pair.into_inner();
        let query = pairs.next().unwrap();
        
        match query.as_rule() { 
            Rule::optional_query => {
                let mut pairs = query.into_inner().next().unwrap().into_inner();
                let key = pairs.next().unwrap();
                let value = pairs.next().unwrap();
                let key_type = KeyType {
                    name: key.as_str().to_string(),
                    var_type: Self::var_type(value),
                };
                (key_type, true)
            }
            Rule::query => {
                let mut pairs = query.into_inner();
                let key = pairs.next().unwrap();
                let value = pairs.next().unwrap();
                let key_type = KeyType {
                    name: key.as_str().to_string(),
                    var_type: Self::var_type(value),
                };
                (key_type, false)
            }
            _ => unimplemented!()
        }
    }
    
    pub fn method(pair: Pair<Rule>) -> Method {
        let mut pairs = pair.into_inner();
        
        let mut pair = pairs.next().unwrap();
        
        let mut url_path = None;
        let mut optional = vec![];
        let mut required = vec![];
        
        while pair.as_rule() == Rule::path_specifier
            || pair.as_rule() == Rule::query_specifier
        {
            match pair.as_rule() {
                Rule::path_specifier => {
                    if url_path.is_some() {
                        panic!("Multiple path specifiers in method");
                    }
                    
                    url_path = Some(Self::path_specifier(pair));
                    
                }
                Rule::query_specifier => {
                    let (key, is_optional) = Self::query_specifier(pair);
                    if is_optional {
                        optional.push(key);
                    } else {
                        required.push(key);
                    }
                }
                _ => unimplemented!()
            }
            
            pair = pairs.next().unwrap();
        }
        
        let method_type = Self::method_type(pair);
        let name = pairs.next().unwrap().as_str().to_string();
        
        let mut pair = pairs.next().unwrap();
        
        // json payload
        while pair.as_rule() == Rule::typed_arg {
            required.push(Self::typed_arg(pair));
            pair = pairs.next().unwrap();
        }
        
        let body = Self::block(pair);
        
        Method {
            method: method_type,
            path: name,
            handler: Handler {
                path: url_path.unwrap(),
                required_query_params: required,
                optional_query_params: optional,
                body,
            }
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
    pub fn data(data: &str) -> Vec<Construct> {
        let pairs = BareParser::parse(Rule::program, data)
            .unwrap_or_else(|e| panic!("{}", e));


        let mut constructs = vec![];

        for pair in pairs {
            match pair.as_rule() {
                Rule::function => {
                    let func = Self::function(pair);
                    constructs.push(Construct::Function(func));
                }
                Rule::method => {
                    let method = Self::method(pair);
                    constructs.push(Construct::Method(method));
                }
                Rule::EOI => {
                    break;
                }
                _ => unimplemented!()
            }
        }

        constructs
    }
}