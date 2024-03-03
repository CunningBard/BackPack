use bpcore::ast::{Block, Construct, Expression, Function, Identifier, Program, Statement, Type, VarType};

const ESSENTIALS: &str = r#"package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

type Option[T any] struct {
	Value T
	Valid bool
}

func NewSome[T any](value T) Option[T] {
	return Option[T]{Value: value, Valid: true}
}

func NewNone[T any]() Option[T] {
	return Option[T]{Valid: false}
}

func (opt Option[T]) Unwrap() T {
	if !opt.Valid {
		panic("attempted to unwrap a None option")
	}
	return opt.Value
}

func input(to_print string) Option[string] {
    fmt.Print(to_print)
	reader := bufio.NewReader(os.Stdin)
	// ReadString will block until the delimiter is entered
	input, err := reader.ReadString('\n')
	if err != nil {
		fmt.Println("An error occured while reading input. Please try again", err)
		return NewNone[string]()
	}

	// remove the delimeter from the string
	input = strings.TrimSuffix(input, "\n")
	return NewSome(input)
}

"#;
pub trait ToGoCode {
    fn to_go_code(&self) -> String;


    // optional to_go_code_formatted
    fn to_go_code_formatted(&self) -> String {
        self.to_go_code()
    }
}

pub fn function_call_translation(name: &Identifier, args: &Vec<Expression>) -> String {
    let binding = name.to_string();
    let print_name = match &*binding {
        "print" => "fmt.Print",
        "println" => "fmt.Println",
        // "input" => "fmt.Scanln",
        unknown => unknown,
    };

    let mut args_str = String::new();
    for arg in args {
        args_str.push_str(&arg.to_go_code());
        args_str.push_str(", ");
    }

    for _ in 0..2 {
        args_str.pop();
    }

    format!("{}({})", print_name, args_str)
}

impl ToGoCode for Expression {
    fn to_go_code(&self) -> String {
        match self {
            Expression::Int(val) => format!("{}", val),
            Expression::Float(val) => format!("{}", val),
            Expression::String(val) => format!("{}", val),
            Expression::Bool(val) => format!("{}", val),
            Expression::Variable(val) => format!("{}", val),

            Expression::Addition { left, right } => format!("{} + {}", left.to_go_code(), right.to_go_code()),
            Expression::Subtraction { left, right } => format!("{} - {}", left.to_go_code(), right.to_go_code()),
            Expression::Multiplication { left, right } => format!("{} * {}", left.to_go_code(), right.to_go_code()),
            Expression::Division { left, right } => format!("{} / {}", left.to_go_code(), right.to_go_code()),
            Expression::Equal { left, right } => format!("{} == {}", left.to_go_code(), right.to_go_code()),
            Expression::NotEqual { left, right } => format!("{} != {}", left.to_go_code(), right.to_go_code()),
            Expression::GreaterThan { left, right } => format!("{} > {}", left.to_go_code(), right.to_go_code()),
            Expression::LessThan { left, right } => format!("{} < {}", left.to_go_code(), right.to_go_code()),
            Expression::GreaterThanOrEqual { left, right } => format!("{} >= {}", left.to_go_code(), right.to_go_code()),
            Expression::LessThanOrEqual { left, right } => format!("{} <= {}", left.to_go_code(), right.to_go_code()),

            Expression::FunctionCall { func_name: name, args } => {
                function_call_translation(name, args)
            },
        }
    }
}

impl ToGoCode for Type {
    fn to_go_code(&self) -> String {
        match self {
            Type::Int => "int".to_string(),
            Type::Float => "float32".to_string(),
            Type::String => "string".to_string(),
            Type::Bool => "bool".to_string(),
        }
    }
}

impl ToGoCode for VarType {
    fn to_go_code(&self) -> String {
        let pointers = "*".repeat(self.pointer_count);
        format!("{}{}", pointers, self.var_type.to_go_code())
    }
}

impl ToGoCode for Function {
    fn to_go_code(&self) -> String {
        let mut args_str = String::new();
        for arg in &self.args {
            args_str.push_str(&format!("{} {}", arg.var_type.to_go_code(), arg.name));
            args_str.push_str(", ");
        }

        for _ in 0..2 {
            args_str.pop();
        }

        if args_str.len() > 0 {
            args_str.pop();
            args_str.pop();
        }

        let body = self.body.to_go_code_formatted();

        match &self.return_type {
            Some(var_type) => {
                format!("func {}({}) {} {}", self.name, args_str, var_type.to_go_code(), body)
            },
            None => {
                format!("func {}({}) {}", self.name, args_str, body)
            }
        }
    }
}

impl ToGoCode for Statement {
    fn to_go_code(&self) -> String {
        match self {
            Statement::VariableAssignment { name, var_type, value } => {
                let var_type = match var_type {
                    Some(var_type) => var_type,
                    None => panic!("Translation: you need to infer type (inference not implemented), variable '{}'", name)
                };

                format!("var {} {} = {}", name, var_type.to_go_code(), value.to_go_code())
            },
            Statement::FunctionCall { name, args } => {
                function_call_translation(name, args)
            },
            Statement::Return { value } => {
                format!("return {}", value.to_go_code())
            },
            Statement::Conditional { conditions, else_body } => {
                let mut conditions_str = "if ".to_string();
                let condition_count = conditions.len();
                for (index, (condition, body)) in conditions.iter().enumerate() {
                    let mut body = body.to_go_code_formatted();
                    if body.len() > 4 {
                        body.push(' ');
                    }
                    conditions_str.push_str(&format!("{} {}else if ", condition.to_go_code(), body));


                    if condition_count - 1 == index {
                        for _ in 0..8 {
                            conditions_str.pop();
                        }

                        if else_body.is_some() {
                            if body.len() < 4 {
                                conditions_str.push(' ');
                            }

                            conditions_str.push_str("else ");
                        }
                    }
                }

                if let Some(else_body) = else_body {
                    conditions_str.push_str(&format!("{}", else_body.to_go_code_formatted()));
                }

                conditions_str
            },
            Statement::WhileLoop { condition, body } => {
                format!("for {} {}", condition.to_go_code(), body.to_go_code_formatted())
            },
            Statement::VariableReassignment { variable_name: name, value } => {
                format!("{} = {}", name, value.to_go_code())
            },
            Statement::Response { .. } => {
                unimplemented!()
            }
        }
    }
}

impl ToGoCode for Block {
    fn to_go_code(&self) -> String {
        self.iter().map(|statement| statement.to_go_code()).collect::<Vec<String>>().join("\n")
    }

    fn to_go_code_formatted(&self) -> String {
        let mut code = String::new();
        
        let go_code = self.to_go_code();

        if go_code.len() == 0 {
            return "{ }\n".to_string()
        }

        for statement in go_code.split("\n") {
            code.push_str("\t");
            code.push_str(&statement);
            code.push_str("\n");
        }

        format!("{{\n{}}}", code)
    }
}

impl ToGoCode for Construct {
    fn to_go_code(&self) -> String {
        match self {
            Construct::Function(function) => function.to_go_code(),
            Construct::Method(_method) => {
                // statement.to_go_code()

                unimplemented!()
            }
        }
    }

}

impl ToGoCode for Program {
    fn to_go_code(&self) -> String {
        format!("{}{}", ESSENTIALS, self.iter().map(|construct| construct.to_go_code()).collect::<Vec<String>>().join("\n"))
    }
}
