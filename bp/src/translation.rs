use bpcore::ast::{
    Block, Construct, Expression, Function, Identifier, KeyType, Method, MethodType, PathType,
    Program, Statement, Type, UrlPath, VarType,
};

const HTTP_ESSENTIALS: &str = r#"package main

import (
	"fmt"
	"net/http"
    "strconv"
)

var _ = strconv.IntSize

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

func (opt Option[T]) unwrap_or(defaultValue T) T {
    if opt.Valid {
        return opt.Value
    }

    return defaultValue
}

"#;

pub trait ToGoCode {
    fn to_go_code(&self) -> String;

    // optional to_go_code_formatted
    fn to_go_code_formatted(&self) -> String {
        self.to_go_code()
    }
}

pub fn function_call_translation(name: &Identifier, args: &[Expression]) -> String {
    let binding = name.to_string();
    let print_name = match &*binding {
        "print" => "fmt.Print",
        "println" => "fmt.Println",
        "format" => "fmt.Sprintf",
        unknown => unknown,
    };

    let args_str = args
        .iter()
        .map(ToGoCode::to_go_code)
        .collect::<Vec<_>>()
        .join(", ");

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
            Expression::And { left, right } => format!("{} && {}", left.to_go_code(), right.to_go_code()),
            Expression::Or { left, right } => format!("{} || {}", left.to_go_code(), right.to_go_code()),

            Expression::FunctionCall { func_name: name, args } => {
                function_call_translation(name, args)
            }
            Expression::Not { expr } => format!("!({})", expr.to_go_code()),
            Expression::Negate { expr } => format!("-({})", expr.to_go_code()),
            Expression::List(items) => {
                let items_str = items
                    .iter()
                    .map(ToGoCode::to_go_code)
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{{{}}}", items_str)
            }
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
            Type::List(inner) => format!("[]{}", inner.to_go_code()),
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
        let args_str = self
            .args
            .iter()
            .map(|arg| format!("{} {}", arg.var_type.to_go_code(), arg.name))
            .collect::<Vec<_>>()
            .join(", ");

        let body = self.body.to_go_code_formatted();

        match &self.return_type {
            Some(var_type) => {
                format!("func {}({}) {} {}", self.name, args_str, var_type.to_go_code(), body)
            }
            None => {
                format!("func {}({}) {}", self.name, args_str, body)
            }
        }
    }
}

impl ToGoCode for PathType {
    fn to_go_code(&self) -> String {
        match self {
            PathType::Path(s) => s.clone(),
            PathType::Variable(s) => format!("{{{}}}", s),
        }
    }
}

impl ToGoCode for UrlPath {
    fn to_go_code(&self) -> String {
        self.path.iter().map(|p| p.to_go_code()).collect::<Vec<_>>().join("/")
    }
}

impl ToGoCode for MethodType {
    fn to_go_code(&self) -> String {
        match self {
            MethodType::Get => "GET".to_string(),
            MethodType::Post => "POST".to_string(),
            MethodType::Put => "PUT".to_string(),
            MethodType::Delete => "DELETE".to_string(),
            MethodType::Patch => "PATCH".to_string(),
        }
    }
}

impl ToGoCode for Method {
    fn to_go_code(&self) -> String {
        let handler_name = format!("{}Handler", self.path);
        let mut body_parts = Vec::new();

        let path_prelude = build_path_prelude(&self.handler.path);
        if !path_prelude.is_empty() {
            body_parts.push(path_prelude);
        }

        let query_prelude = build_query_prelude(
            &self.handler.required_query_params,
            &self.handler.optional_query_params,
        );
        if !query_prelude.is_empty() {
            body_parts.push(query_prelude);
        }

        let header_prelude = build_header_prelude(
            &self.handler.required_headers,
            &self.handler.expected_headers,
        );
        if !header_prelude.is_empty() {
            body_parts.push(header_prelude);
        }

        let handler_body = self.handler.body.to_go_code();
        if !handler_body.is_empty() {
            body_parts.push(handler_body);
        }

        let body = format_go_block(&body_parts.join("\n"));
        format!("func {}(w http.ResponseWriter, r *http.Request) {}", handler_name, body)
    }
}

fn format_go_block(go_code: &str) -> String {
    if go_code.is_empty() {
        return "{ }\n".to_string();
    }

    let code = go_code
        .lines()
        .map(|statement| format!("\t{}", statement))
        .collect::<Vec<_>>()
        .join("\n");

    format!("{{\n{}\n}}", code)
}

fn build_path_prelude(url_path: &UrlPath) -> String {
    let lines: Vec<String> = url_path
        .path
        .iter()
        .filter_map(|p| match p {
            PathType::Variable(name) => Some(vec![
                format!("{} := r.PathValue(\"{}\")", name, name),
                format!("_ = {}", name),
            ]),
            PathType::Path(_) => None,
        })
        .flatten()
        .collect();

    lines.join("\n")
}

fn build_query_prelude(required: &[KeyType], optional: &[KeyType]) -> String {
    if required.is_empty() && optional.is_empty() {
        return String::new();
    }

    let mut lines = vec!["query := r.URL.Query()".to_string()];

    for key in required {
        lines.extend(query_param_lines(key, false));
    }

    for key in optional {
        lines.extend(query_param_lines(key, true));
    }

    lines.join("\n")
}

fn build_header_prelude(required_headers: &[String], expected_headers: &[(String, String)]) -> String {
    if required_headers.is_empty() && expected_headers.is_empty() {
        return String::new();
    }

    let mut lines = Vec::new();

    for header in required_headers {
        lines.push(format!("{} := r.Header.Get(\"{}\")", header, header));
        lines.push(format!("if {} == \"\" {{", header));
        lines.push(format!("\thttp.Error(w, \"missing header: {}\", http.StatusBadRequest)", header));
        lines.push("\treturn".to_string());
        lines.push("}".to_string());
        lines.push(format!("_ = {}", header));
    }

    for (header, expected) in expected_headers {
        lines.push(format!("actual_{} := r.Header.Get(\"{}\")", header, header));
        lines.push(format!("if actual_{} != \"{}\" {{", header, expected));
        lines.push(format!("\thttp.Error(w, \"header {} must be {}\", http.StatusBadRequest)", header, expected));
        lines.push("\treturn".to_string());
        lines.push("}".to_string());
    }

    lines.join("\n")
}

fn query_param_lines(key: &KeyType, optional: bool) -> Vec<String> {
    let name = key.name.as_str();
    let raw_name = format!("raw_{}", name);
    let err_name = format!("err_{}", name);

    let mut lines = vec![format!("{} := query.Get(\"{}\")", raw_name, name)];

    if !optional {
        lines.push(format!("if {} == \"\" {{", raw_name));
        lines.push(format!("\thttp.Error(w, \"missing query param: {}\", http.StatusBadRequest)", name));
        lines.push("\treturn".to_string());
        lines.push("}".to_string());
    }

    match key.var_type.var_type {
        Type::String => {
            if optional {
                lines.push(format!("{} := NewNone[string]()", name));
                lines.push(format!("if {} != \"\" {{", raw_name));
                lines.push(format!("\t{} = NewSome[string]({})", name, raw_name));
                lines.push("}".to_string());
            } else {
                lines.push(format!("{} := {}", name, raw_name));
            }
        }
        Type::Int => {
            if optional {
                lines.push(format!("{} := NewNone[int]()", name));
                lines.push(format!("if {} != \"\" {{", raw_name));
                lines.push(format!("\tparsed_{}, {} := strconv.Atoi({})", name, err_name, raw_name));
                lines.push(format!("\tif {} != nil {{", err_name));
                lines.push(format!("\t\thttp.Error(w, \"invalid query param: {}\", http.StatusBadRequest)", name));
                lines.push("\t\treturn".to_string());
                lines.push("\t}".to_string());
                lines.push(format!("\t{} = NewSome[int](parsed_{})", name, name));
                lines.push("}".to_string());
            } else {
                lines.push(format!("{}, {} := strconv.Atoi({})", name, err_name, raw_name));
                lines.push(format!("if {} != nil {{", err_name));
                lines.push(format!("\thttp.Error(w, \"invalid query param: {}\", http.StatusBadRequest)", name));
                lines.push("\treturn".to_string());
                lines.push("}".to_string());
            }
        }
        Type::Float => {
            if optional {
                lines.push(format!("{} := NewNone[float32]()", name));
                lines.push(format!("if {} != \"\" {{", raw_name));
                lines.push(format!("\tparsed_{}, {} := strconv.ParseFloat({}, 32)", name, err_name, raw_name));
                lines.push(format!("\tif {} != nil {{", err_name));
                lines.push(format!("\t\thttp.Error(w, \"invalid query param: {}\", http.StatusBadRequest)", name));
                lines.push("\t\treturn".to_string());
                lines.push("\t}".to_string());
                lines.push(format!("\t{} = NewSome[float32](float32(parsed_{}))", name, name));
                lines.push("}".to_string());
            } else {
                lines.push(format!("parsed_{}, {} := strconv.ParseFloat({}, 32)", name, err_name, raw_name));
                lines.push(format!("if {} != nil {{", err_name));
                lines.push(format!("\thttp.Error(w, \"invalid query param: {}\", http.StatusBadRequest)", name));
                lines.push("\treturn".to_string());
                lines.push("}".to_string());
                lines.push(format!("{} := float32(parsed_{})", name, name));
            }
        }
        Type::Bool => {
            if optional {
                lines.push(format!("{} := NewNone[bool]()", name));
                lines.push(format!("if {} != \"\" {{", raw_name));
                lines.push(format!("\tparsed_{}, {} := strconv.ParseBool({})", name, err_name, raw_name));
                lines.push(format!("\tif {} != nil {{", err_name));
                lines.push(format!("\t\thttp.Error(w, \"invalid query param: {}\", http.StatusBadRequest)", name));
                lines.push("\t\treturn".to_string());
                lines.push("\t}".to_string());
                lines.push(format!("\t{} = NewSome[bool](parsed_{})", name, name));
                lines.push("}".to_string());
            } else {
                lines.push(format!("{}, {} := strconv.ParseBool({})", name, err_name, raw_name));
                lines.push(format!("if {} != nil {{", err_name));
                lines.push(format!("\thttp.Error(w, \"invalid query param: {}\", http.StatusBadRequest)", name));
                lines.push("\treturn".to_string());
                lines.push("}".to_string());
            }
        }
        Type::List(_) => {
            panic!("#QUERY param '{}' cannot be a list type", name);
        }
    }

    // Query variables are generated from specifiers and may be intentionally unused by handler logic.
    lines.push(format!("_ = {}", name));

    lines
}

impl ToGoCode for Statement {
    fn to_go_code(&self) -> String {
        match self {
            Statement::VariableAssignment { name, var_type, value } => {
                let var_type = match var_type {
                    Some(var_type) => var_type,
                    None => panic!(
                        "Translation: you need to infer type (inference not implemented), variable '{}'",
                        name
                    ),
                };

                // Preserve element type in Go for list literals like [str] = ["a", "b"].
                if let (Type::List(inner), Expression::List(items)) = (&var_type.var_type, value) {
                    let items_str = items
                        .iter()
                        .map(ToGoCode::to_go_code)
                        .collect::<Vec<_>>()
                        .join(", ");
                    return format!(
                        "var {} {} = []{}{{{}}}",
                        name,
                        var_type.to_go_code(),
                        inner.to_go_code(),
                        items_str
                    );
                }

                format!("var {} {} = {}", name, var_type.to_go_code(), value.to_go_code())
            }
            Statement::FunctionCall { name, args } => function_call_translation(name, args),
            Statement::Return { value } => format!("return {}", value.to_go_code()),
            Statement::Conditional { conditions, else_body } => {
                let mut output = String::new();

                for (index, (condition, body)) in conditions.iter().enumerate() {
                    let keyword = if index == 0 { "if" } else { "else if" };
                    output.push_str(&format!(
                        "{} {} {}",
                        keyword,
                        condition.to_go_code(),
                        body.to_go_code_formatted()
                    ));
                    if index + 1 < conditions.len() {
                        output.push(' ');
                    }
                }

                if let Some(else_body) = else_body {
                    output.push_str(&format!(" else {}", else_body.to_go_code_formatted()));
                }

                output
            }
            Statement::WhileLoop { condition, body } => {
                format!("for {} {}", condition.to_go_code(), body.to_go_code_formatted())
            }
            Statement::ForLoop { var, iter, body } => {
                format!("for _, {} := range {} {}", var, iter.to_go_code(), body.to_go_code_formatted())
            }
            Statement::VariableReassignment { variable_name: name, value } => {
                format!("{} = {}", name, value.to_go_code())
            }
            Statement::Response { status_code, body } => {
                format!(
                    "w.WriteHeader({})\nfmt.Fprint(w, {})",
                    status_code.to_go_code(),
                    body.to_go_code()
                )
            }
        }
    }
}

impl ToGoCode for Block {
    fn to_go_code(&self) -> String {
        self.iter()
            .map(ToGoCode::to_go_code)
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn to_go_code_formatted(&self) -> String {
        format_go_block(&self.to_go_code())
    }
}

impl ToGoCode for Construct {
    fn to_go_code(&self) -> String {
        match self {
            Construct::Function(function) => function.to_go_code(),
            Construct::Method(method) => method.to_go_code(),
        }
    }
}

impl ToGoCode for Program {
    fn to_go_code(&self) -> String {
        let mut handler_funcs = Vec::new();
        let mut mux_registrations = Vec::new();
        let mut regular_funcs = Vec::new();

        for construct in self {
            match construct {
                Construct::Method(method) => {
                    handler_funcs.push(method.to_go_code());

                    let method_str = method.method.to_go_code();
                    let url = method.handler.path.to_go_code();
                    let handler_name = format!("{}Handler", method.path);
                    mux_registrations.push(format!(
                        "\tmux.HandleFunc(\"{} /{}\", {})",
                        method_str, url, handler_name
                    ));
                }
                Construct::Function(func) => {
                    regular_funcs.push(func.to_go_code());
                }
            }
        }

        let regular_funcs = if regular_funcs.is_empty() {
            String::new()
        } else {
            format!("{}\n\n", regular_funcs.join("\n\n"))
        };

        let handler_funcs = if handler_funcs.is_empty() {
            String::new()
        } else {
            format!("{}\n\n", handler_funcs.join("\n\n"))
        };

        let mux_registrations = mux_registrations.join("\n");

        let main_func = format!(
            "func main() {{\n\tmux := http.NewServeMux()\n\n{}\n\thttp.ListenAndServe(\":8989\", mux)\n}}",
            mux_registrations
        );

        format!("{}{}{}{}", HTTP_ESSENTIALS, regular_funcs, handler_funcs, main_func)
    }
}
