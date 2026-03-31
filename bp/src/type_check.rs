use std::collections::HashMap;

use bpcore::ast::{
    Block, Construct, Expression, Function, Handler, Identifier, Method, Program, Statement, Struct,
    Type, VarType,
};

#[derive(Debug, Clone)]
struct FunctionSig {
    args: Vec<Type>,
    return_type: Option<Type>,
}

#[derive(Debug, Clone)]
enum ExprType {
    Concrete(Type),
    Unit,
    Any,
}

#[derive(Default)]
struct Env {
    scopes: Vec<HashMap<String, Type>>,
}

impl Env {
    fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn insert(&mut self, name: String, ty: Type) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, ty);
        }
    }

    fn get(&self, name: &str) -> Option<Type> {
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Some(ty.clone());
            }
        }
        None
    }
}

fn types_compatible(expected: &Type, actual: &ExprType) -> bool {
    match actual {
        ExprType::Concrete(actual) => same_type(expected, actual),
        ExprType::Any => true,
        ExprType::Unit => false,
    }
}

fn same_type(a: &Type, b: &Type) -> bool {
    match (a, b) {
        (Type::Int, Type::Int)
        | (Type::String, Type::String)
        | (Type::Float, Type::Float)
        | (Type::Bool, Type::Bool) => true,
        (Type::Struct(a), Type::Struct(b)) => a == b,
        (Type::List(a), Type::List(b)) => same_type(a, b),
        _ => false,
    }
}

fn is_numeric(ty: &ExprType) -> bool {
    matches!(ty, ExprType::Concrete(Type::Int) | ExprType::Concrete(Type::Float))
}

fn struct_fields_map(struct_decl: &Struct) -> HashMap<String, Type> {
    struct_decl
        .fields
        .iter()
        .map(|field| (field.name.clone(), field.var_type.var_type.clone()))
        .collect()
}

fn expression_to_type_name(ty: &ExprType) -> String {
    match ty {
        ExprType::Concrete(Type::Int) => "int".to_string(),
        ExprType::Concrete(Type::String) => "str".to_string(),
        ExprType::Concrete(Type::Float) => "float".to_string(),
        ExprType::Concrete(Type::Bool) => "bool".to_string(),
        ExprType::Concrete(Type::Struct(name)) => format!("struct {}", name),
        ExprType::Concrete(Type::List(_)) => "list".to_string(),
        ExprType::Unit => "unit".to_string(),
        ExprType::Any => "any".to_string(),
    }
}

fn is_supported_status_name(name: &str) -> bool {
    matches!(
        name,
        "OK"
            | "CREATED"
            | "NO_CONTENT"
            | "BAD_REQUEST"
            | "UNAUTHORIZED"
            | "FORBIDDEN"
            | "NOT_FOUND"
            | "CONFLICT"
            | "INTERNAL_SERVER_ERROR"
    )
}

pub fn type_check_program(program: &mut Program) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    let mut struct_defs: HashMap<String, HashMap<String, Type>> = HashMap::new();
    let mut function_sigs: HashMap<String, FunctionSig> = HashMap::new();

    for construct in program.iter() {
        match construct {
            Construct::Struct(struct_decl) => {
                if struct_defs.contains_key(&struct_decl.name) {
                    errors.push(format!("duplicate struct declaration '{}'+", struct_decl.name));
                } else {
                    struct_defs.insert(struct_decl.name.clone(), struct_fields_map(struct_decl));
                }
            }
            Construct::Function(function) => {
                if function_sigs.contains_key(&function.name) {
                    errors.push(format!("duplicate function declaration '{}'+", function.name));
                } else {
                    let args = function
                        .args
                        .iter()
                        .map(|arg| arg.var_type.var_type.clone())
                        .collect();
                    let return_type = function.return_type.as_ref().map(|vt| vt.var_type.clone());
                    function_sigs.insert(
                        function.name.clone(),
                        FunctionSig {
                            args,
                            return_type,
                        },
                    );
                }
            }
            Construct::Method(_) => {}
            Construct::Import(_) => {}
            Construct::FromImport(_) => {}
        }
    }

    infer_types_in_program(program, &struct_defs, &function_sigs, &mut errors);

    for construct in program.iter() {
        match construct {
            Construct::Function(function) => check_function(function, &struct_defs, &function_sigs, &mut errors),
            Construct::Method(method) => check_method(method, &struct_defs, &function_sigs, &mut errors),
            Construct::Struct(_) => {}
            Construct::Import(_) => {}
            Construct::FromImport(_) => {}
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn infer_types_in_program(
    program: &mut Program,
    struct_defs: &HashMap<String, HashMap<String, Type>>,
    function_sigs: &HashMap<String, FunctionSig>,
    errors: &mut Vec<String>,
) {
    for construct in program.iter_mut() {
        match construct {
            Construct::Function(function) => infer_types_in_function(function, struct_defs, function_sigs, errors),
            Construct::Method(method) => infer_types_in_method(method, struct_defs, function_sigs, errors),
            Construct::Struct(_) => {}
            Construct::Import(_) => {}
            Construct::FromImport(_) => {}
        }
    }
}

fn infer_types_in_function(
    function: &mut Function,
    struct_defs: &HashMap<String, HashMap<String, Type>>,
    function_sigs: &HashMap<String, FunctionSig>,
    errors: &mut Vec<String>,
) {
    let mut env = Env::new();

    for arg in &function.args {
        env.insert(arg.name.clone(), arg.var_type.var_type.clone());
    }

    infer_types_in_block(&mut function.body, &mut env, struct_defs, function_sigs, errors);
}

fn infer_types_in_method(
    method: &mut Method,
    struct_defs: &HashMap<String, HashMap<String, Type>>,
    function_sigs: &HashMap<String, FunctionSig>,
    errors: &mut Vec<String>,
) {
    let mut env = Env::new();
    seed_method_env(&method.handler, &mut env);

    infer_types_in_block(
        &mut method.handler.body,
        &mut env,
        struct_defs,
        function_sigs,
        errors,
    );
}

fn infer_types_in_block(
    block: &mut Block,
    env: &mut Env,
    struct_defs: &HashMap<String, HashMap<String, Type>>,
    function_sigs: &HashMap<String, FunctionSig>,
    errors: &mut Vec<String>,
) {
    env.push_scope();

    for statement in block.iter_mut() {
        match statement {
            Statement::VariableAssignment {
                name,
                var_type,
                value,
            } => {
                if var_type.is_none() {
                    let value_type = infer_expr(value, env, struct_defs, function_sigs, errors);
                    if let ExprType::Concrete(ty) = value_type {
                        *var_type = Some(VarType {
                            pointer_count: 0,
                            var_type: ty.clone(),
                        });
                        env.insert(name.clone(), ty);
                    }
                } else if let Some(vt) = var_type {
                    env.insert(name.clone(), vt.var_type.clone());
                }
            }
            Statement::VariableReassignment { .. } => {
                // No type inference needed for reassignment
            }
            Statement::FunctionCall { .. } => {
                // No type mutations needed
            }
            Statement::Return { .. } => {
                // No type inference needed for return
            }
            Statement::Response { .. } => {
                // No type inference needed
            }
            Statement::Conditional {
                conditions,
                else_body,
            } => {
                for (_, cond_body) in conditions.iter_mut() {
                    infer_types_in_block(cond_body, env, struct_defs, function_sigs, errors);
                }
                if let Some(else_body) = else_body {
                    infer_types_in_block(else_body, env, struct_defs, function_sigs, errors);
                }
            }
            Statement::WhileLoop { body, .. } => {
                infer_types_in_block(body, env, struct_defs, function_sigs, errors);
            }
            Statement::ForLoop { var: _, body, .. } => {
                env.push_scope();
                // For loop variable will be typed by check_block
                infer_types_in_block(body, env, struct_defs, function_sigs, errors);
                env.pop_scope();
            }
            Statement::IfSome {
                some_body,
                none_body,
                ..
            } => {
                env.push_scope();
                infer_types_in_block(some_body, env, struct_defs, function_sigs, errors);
                env.pop_scope();

                if let Some(none_body) = none_body {
                    infer_types_in_block(none_body, env, struct_defs, function_sigs, errors);
                }
            }
        }
    }

    env.pop_scope();
}

fn check_function(
    function: &Function,
    struct_defs: &HashMap<String, HashMap<String, Type>>,
    function_sigs: &HashMap<String, FunctionSig>,
    errors: &mut Vec<String>,
) {
    let mut env = Env::new();

    for arg in &function.args {
        env.insert(arg.name.clone(), arg.var_type.var_type.clone());
    }

    let expected_return = function.return_type.as_ref().map(|vt| vt.var_type.clone());
    check_block(
        &function.body,
        &mut env,
        expected_return.as_ref(),
        false,
        struct_defs,
        function_sigs,
        errors,
    );
}

fn check_method(
    method: &Method,
    struct_defs: &HashMap<String, HashMap<String, Type>>,
    function_sigs: &HashMap<String, FunctionSig>,
    errors: &mut Vec<String>,
) {
    let mut env = Env::new();
    seed_method_env(&method.handler, &mut env);

    check_block(
        &method.handler.body,
        &mut env,
        None,
        true,
        struct_defs,
        function_sigs,
        errors,
    );
}

fn seed_method_env(handler: &Handler, env: &mut Env) {
    for key in &handler.required_query_params {
        env.insert(key.name.clone(), key.var_type.var_type.clone());
    }

    for key in &handler.optional_query_params {
        env.insert(key.name.clone(), key.var_type.var_type.clone());
    }

    for key in &handler.payload_params {
        env.insert(key.name.clone(), key.var_type.var_type.clone());
    }

    for path_part in &handler.path.path {
        if let bpcore::ast::PathType::Variable(name) = path_part {
            env.insert(name.clone(), Type::String);
        }
    }

    for header in &handler.required_headers {
        env.insert(header.clone(), Type::String);
    }
}

fn check_block(
    block: &Block,
    env: &mut Env,
    expected_return: Option<&Type>,
    in_method: bool,
    struct_defs: &HashMap<String, HashMap<String, Type>>,
    function_sigs: &HashMap<String, FunctionSig>,
    errors: &mut Vec<String>,
) {
    env.push_scope();

    for statement in block {
        match statement {
            Statement::VariableAssignment {
                name,
                var_type,
                value,
            } => {
                let value_type = infer_expr(value, env, struct_defs, function_sigs, errors);

                match var_type {
                    Some(var_type) => {
                        let expected = &var_type.var_type;
                        if !types_compatible(expected, &value_type) {
                            errors.push(format!(
                                "type mismatch in assignment to '{}': expected {}, got {}",
                                name,
                                expected.type_name(),
                                expression_to_type_name(&value_type)
                            ));
                        }
                        env.insert(name.clone(), expected.clone());
                    }
                    None => {
                        match value_type {
                            ExprType::Concrete(ty) => env.insert(name.clone(), ty),
                            ExprType::Any => {
                                errors.push(format!(
                                    "cannot infer type for '{}' from expression of unknown type",
                                    name
                                ));
                            }
                            ExprType::Unit => {
                                errors.push(format!(
                                    "cannot assign unit value to '{}' without an explicit type",
                                    name
                                ));
                            }
                        }
                    }
                }
            }
            Statement::VariableReassignment {
                variable_name,
                value,
            } => {
                let target_type = resolve_identifier_type(variable_name, env, struct_defs, errors);
                let value_type = infer_expr(value, env, struct_defs, function_sigs, errors);

                if let Some(target_type) = target_type {
                    if !types_compatible(&target_type, &value_type) {
                        errors.push(format!(
                            "type mismatch in reassignment '{}': expected {}, got {}",
                            variable_name,
                            target_type.type_name(),
                            expression_to_type_name(&value_type)
                        ));
                    }
                }
            }
            Statement::FunctionCall { name, args } => {
                let expr = Expression::FunctionCall {
                    func_name: name.clone(),
                    args: args.clone(),
                };
                let _ = infer_expr(&expr, env, struct_defs, function_sigs, errors);
            }
            Statement::Return { value } => {
                if in_method {
                    errors.push("'return' is not allowed inside methods; use respond".to_string());
                    continue;
                }

                let value_type = infer_expr(value, env, struct_defs, function_sigs, errors);
                match expected_return {
                    Some(expected_type) => {
                        if !types_compatible(expected_type, &value_type) {
                            errors.push(format!(
                                "return type mismatch: expected {}, got {}",
                                expected_type.type_name(),
                                expression_to_type_name(&value_type)
                            ));
                        }
                    }
                    None => {
                        errors.push("return used in function with no return type".to_string());
                    }
                }
            }
            Statement::Response { status_code, body } => {
                let status_type = infer_expr(status_code, env, struct_defs, function_sigs, errors);
                if !matches!(status_type, ExprType::Concrete(Type::Int) | ExprType::Any) {
                    errors.push(format!(
                        "respond status code must be int, got {}",
                        expression_to_type_name(&status_type)
                    ));
                }
                let _ = infer_expr(body, env, struct_defs, function_sigs, errors);
            }
            Statement::Conditional {
                conditions,
                else_body,
            } => {
                for (condition, cond_body) in conditions {
                    let cond_type = infer_expr(condition, env, struct_defs, function_sigs, errors);
                    if !matches!(cond_type, ExprType::Concrete(Type::Bool) | ExprType::Any) {
                        errors.push(format!(
                            "if condition must be bool, got {}",
                            expression_to_type_name(&cond_type)
                        ));
                    }

                    check_block(
                        cond_body,
                        env,
                        expected_return,
                        in_method,
                        struct_defs,
                        function_sigs,
                        errors,
                    );
                }

                if let Some(else_body) = else_body {
                    check_block(
                        else_body,
                        env,
                        expected_return,
                        in_method,
                        struct_defs,
                        function_sigs,
                        errors,
                    );
                }
            }
            Statement::WhileLoop { condition, body } => {
                let cond_type = infer_expr(condition, env, struct_defs, function_sigs, errors);
                if !matches!(cond_type, ExprType::Concrete(Type::Bool) | ExprType::Any) {
                    errors.push(format!(
                        "while condition must be bool, got {}",
                        expression_to_type_name(&cond_type)
                    ));
                }

                check_block(
                    body,
                    env,
                    expected_return,
                    in_method,
                    struct_defs,
                    function_sigs,
                    errors,
                );
            }
            Statement::ForLoop { var, iter, body } => {
                let iter_type = infer_expr(iter, env, struct_defs, function_sigs, errors);

                let item_type = match iter_type {
                    ExprType::Concrete(Type::List(inner)) => Some(*inner),
                    ExprType::Any => None,
                    other => {
                        errors.push(format!(
                            "for loop iterable must be a list, got {}",
                            expression_to_type_name(&other)
                        ));
                        None
                    }
                };

                env.push_scope();
                if let Some(item_type) = item_type {
                    env.insert(var.clone(), item_type);
                }
                check_block(
                    body,
                    env,
                    expected_return,
                    in_method,
                    struct_defs,
                    function_sigs,
                    errors,
                );
                env.pop_scope();
            }
            Statement::IfSome {
                binding,
                source,
                some_body,
                none_body,
            } => {
                let source_type = infer_expr(source, env, struct_defs, function_sigs, errors);

                env.push_scope();
                if let ExprType::Concrete(ty) = source_type {
                    env.insert(binding.clone(), ty);
                }
                check_block(
                    some_body,
                    env,
                    expected_return,
                    in_method,
                    struct_defs,
                    function_sigs,
                    errors,
                );
                env.pop_scope();

                if let Some(none_body) = none_body {
                    check_block(
                        none_body,
                        env,
                        expected_return,
                        in_method,
                        struct_defs,
                        function_sigs,
                        errors,
                    );
                }
            }
        }
    }

    env.pop_scope();
}

fn infer_expr(
    expr: &Expression,
    env: &mut Env,
    struct_defs: &HashMap<String, HashMap<String, Type>>,
    function_sigs: &HashMap<String, FunctionSig>,
    errors: &mut Vec<String>,
) -> ExprType {
    match expr {
        Expression::Variable(identifier) => resolve_identifier_type(identifier, env, struct_defs, errors)
            .map(ExprType::Concrete)
            .unwrap_or(ExprType::Any),
        Expression::String(_) => ExprType::Concrete(Type::String),
        Expression::Int(_) => ExprType::Concrete(Type::Int),
        Expression::Float(_) => ExprType::Concrete(Type::Float),
        Expression::Bool(_) => ExprType::Concrete(Type::Bool),
        Expression::List(items) => {
            if items.is_empty() {
                return ExprType::Any;
            }

            let first = infer_expr(&items[0], env, struct_defs, function_sigs, errors);
            for item in items.iter().skip(1) {
                let next = infer_expr(item, env, struct_defs, function_sigs, errors);
                match (&first, &next) {
                    (ExprType::Concrete(a), ExprType::Concrete(b)) if same_type(a, b) => {}
                    (ExprType::Any, _) | (_, ExprType::Any) => {}
                    _ => errors.push("list literal contains mixed element types".to_string()),
                }
            }

            match first {
                ExprType::Concrete(ty) => ExprType::Concrete(Type::List(Box::new(ty))),
                _ => ExprType::Any,
            }
        }
        Expression::StructLiteral(fields) => {
            for (_, value) in fields {
                let _ = infer_expr(value, env, struct_defs, function_sigs, errors);
            }
            ExprType::Any
        }
        Expression::TypedStructLiteral {
            struct_name,
            fields,
        } => {
            let Some(struct_fields) = struct_defs.get(struct_name) else {
                errors.push(format!("unknown struct '{}' in literal", struct_name));
                return ExprType::Any;
            };

            let mut seen = HashMap::<String, bool>::new();

            for (field_name, value) in fields {
                seen.insert(field_name.clone(), true);
                let value_type = infer_expr(value, env, struct_defs, function_sigs, errors);

                match struct_fields.get(field_name) {
                    Some(expected_type) => {
                        if !types_compatible(expected_type, &value_type) {
                            errors.push(format!(
                                "field '{}.{}' expected {}, got {}",
                                struct_name,
                                field_name,
                                expected_type.type_name(),
                                expression_to_type_name(&value_type)
                            ));
                        }
                    }
                    None => errors.push(format!(
                        "unknown field '{}.{}' in struct literal",
                        struct_name, field_name
                    )),
                }
            }

            for expected_name in struct_fields.keys() {
                if !seen.contains_key(expected_name) {
                    errors.push(format!(
                        "missing field '{}.{}' in struct literal",
                        struct_name, expected_name
                    ));
                }
            }

            ExprType::Concrete(Type::Struct(struct_name.clone()))
        }
        Expression::Addition { left, right }
        | Expression::Subtraction { left, right }
        | Expression::Multiplication { left, right }
        | Expression::Division { left, right } => {
            let left_ty = infer_expr(left, env, struct_defs, function_sigs, errors);
            let right_ty = infer_expr(right, env, struct_defs, function_sigs, errors);

            if is_numeric(&left_ty) && is_numeric(&right_ty) {
                if matches!(left_ty, ExprType::Concrete(Type::Float))
                    || matches!(right_ty, ExprType::Concrete(Type::Float))
                {
                    ExprType::Concrete(Type::Float)
                } else {
                    ExprType::Concrete(Type::Int)
                }
            } else if matches!(expr, Expression::Addition { .. })
                && matches!(left_ty, ExprType::Concrete(Type::String))
                && matches!(right_ty, ExprType::Concrete(Type::String))
            {
                ExprType::Concrete(Type::String)
            } else {
                errors.push(format!(
                    "arithmetic operation requires numeric types, got {} and {}",
                    expression_to_type_name(&left_ty),
                    expression_to_type_name(&right_ty)
                ));
                ExprType::Any
            }
        }
        Expression::Equal { left, right }
        | Expression::NotEqual { left, right }
        | Expression::GreaterThan { left, right }
        | Expression::LessThan { left, right }
        | Expression::GreaterThanOrEqual { left, right }
        | Expression::LessThanOrEqual { left, right } => {
            let left_ty = infer_expr(left, env, struct_defs, function_sigs, errors);
            let right_ty = infer_expr(right, env, struct_defs, function_sigs, errors);

            match (&left_ty, &right_ty) {
                (ExprType::Concrete(a), ExprType::Concrete(b)) if same_type(a, b) => {
                    ExprType::Concrete(Type::Bool)
                }
                (ExprType::Any, _) | (_, ExprType::Any) => ExprType::Concrete(Type::Bool),
                _ => {
                    errors.push(format!(
                        "comparison between incompatible types: {} and {}",
                        expression_to_type_name(&left_ty),
                        expression_to_type_name(&right_ty)
                    ));
                    ExprType::Concrete(Type::Bool)
                }
            }
        }
        Expression::And { left, right } | Expression::Or { left, right } => {
            let left_ty = infer_expr(left, env, struct_defs, function_sigs, errors);
            let right_ty = infer_expr(right, env, struct_defs, function_sigs, errors);

            let left_ok = matches!(left_ty, ExprType::Concrete(Type::Bool) | ExprType::Any);
            let right_ok = matches!(right_ty, ExprType::Concrete(Type::Bool) | ExprType::Any);

            if !left_ok || !right_ok {
                errors.push(format!(
                    "logical operators require bool operands, got {} and {}",
                    expression_to_type_name(&left_ty),
                    expression_to_type_name(&right_ty)
                ));
            }

            ExprType::Concrete(Type::Bool)
        }
        Expression::Not { expr } => {
            let expr_ty = infer_expr(expr, env, struct_defs, function_sigs, errors);
            if !matches!(expr_ty, ExprType::Concrete(Type::Bool) | ExprType::Any) {
                errors.push(format!(
                    "'!' requires bool operand, got {}",
                    expression_to_type_name(&expr_ty)
                ));
            }
            ExprType::Concrete(Type::Bool)
        }
        Expression::Negate { expr } => {
            let expr_ty = infer_expr(expr, env, struct_defs, function_sigs, errors);
            if !is_numeric(&expr_ty) && !matches!(expr_ty, ExprType::Any) {
                errors.push(format!(
                    "unary '-' requires numeric operand, got {}",
                    expression_to_type_name(&expr_ty)
                ));
            }
            expr_ty
        }
        Expression::FunctionCall { func_name, args } => {
            for arg in args {
                let _ = infer_expr(arg, env, struct_defs, function_sigs, errors);
            }

            if let Identifier::Single(name) = func_name {
                match name.as_str() {
                    "print" | "println" => return ExprType::Unit,
                    "format" => {
                        if args.is_empty() {
                            errors.push("format(...) requires at least one argument".to_string());
                            return ExprType::Concrete(Type::String);
                        }

                        let first = infer_expr(&args[0], env, struct_defs, function_sigs, errors);
                        if !matches!(first, ExprType::Concrete(Type::String) | ExprType::Any) {
                            errors.push(format!(
                                "format first arg must be str, got {}",
                                expression_to_type_name(&first)
                            ));
                        }

                        return ExprType::Concrete(Type::String);
                    }
                    _ => {}
                }

                if let Some(sig) = function_sigs.get(name) {
                    if sig.args.len() != args.len() {
                        errors.push(format!(
                            "function '{}' expects {} args, got {}",
                            name,
                            sig.args.len(),
                            args.len()
                        ));
                    }

                    for (idx, expected) in sig.args.iter().enumerate() {
                        if let Some(actual_expr) = args.get(idx) {
                            let actual = infer_expr(actual_expr, env, struct_defs, function_sigs, errors);
                            if !types_compatible(expected, &actual) {
                                errors.push(format!(
                                    "arg {} of '{}' expected {}, got {}",
                                    idx + 1,
                                    name,
                                    expected.type_name(),
                                    expression_to_type_name(&actual)
                                ));
                            }
                        }
                    }

                    return sig
                        .return_type
                        .clone()
                        .map(ExprType::Concrete)
                        .unwrap_or(ExprType::Unit);
                }
            }

            ExprType::Any
        }
    }
}

fn resolve_identifier_type(
    identifier: &Identifier,
    env: &Env,
    struct_defs: &HashMap<String, HashMap<String, Type>>,
    errors: &mut Vec<String>,
) -> Option<Type> {
    match identifier {
        Identifier::Single(name) => env.get(name).or_else(|| {
            errors.push(format!("unknown identifier '{}'", name));
            None
        }),
        Identifier::DotIdentifier(parts) => {
            if parts.is_empty() {
                return None;
            }

            if parts[0] == "Status" {
                if parts.len() != 2 {
                    errors.push(format!(
                        "invalid status constant '{}': expected Status.<NAME>",
                        parts.join(".")
                    ));
                    return None;
                }

                if !is_supported_status_name(&parts[1]) {
                    errors.push(format!("unsupported status constant 'Status.{}'", parts[1]));
                    return None;
                }

                return Some(Type::Int);
            }

            let mut current = match env.get(&parts[0]) {
                Some(ty) => ty,
                None => {
                    errors.push(format!("unknown identifier '{}'", parts[0]));
                    return None;
                }
            };

            for field in parts.iter().skip(1) {
                match current {
                    Type::Struct(ref struct_name) => {
                        let Some(fields) = struct_defs.get(struct_name) else {
                            errors.push(format!("unknown struct type '{}'+", struct_name));
                            return None;
                        };

                        let Some(next) = fields.get(field) else {
                            errors.push(format!(
                                "struct '{}' has no field '{}'",
                                struct_name, field
                            ));
                            return None;
                        };
                        current = next.clone();
                    }
                    _ => {
                        errors.push(format!(
                            "field access '{}' requires a struct value",
                            parts.join(".")
                        ));
                        return None;
                    }
                }
            }

            Some(current)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::type_check_program;
    use crate::parser::BpParse;

    #[test]
    fn type_check_accepts_struct_payload_and_literal() {
        let source = r#"
struct Person {
    name: str,
    age: int,
}

func mk() -> Person {
    let p: Person = Person { name: "jack", age: 5 };
    return p;
}

GET say_hello(person: Person) {
    respond 200 person.name;
}
"#;

        let mut program = BpParse::data(source);
        let result = type_check_program(&mut program);
        assert!(result.is_ok(), "expected ok, got: {:?}", result.err());
    }

    #[test]
    fn type_check_rejects_struct_field_type_mismatch() {
        let source = r#"
struct Person {
    name: str,
    age: int,
}

func mk() -> Person {
    let p: Person = Person { name: "jack", age: "oops" };
    return p;
}
"#;

        let mut program = BpParse::data(source);
        let result = type_check_program(&mut program);
        assert!(result.is_err());
        let errors = result.err().unwrap();
        assert!(
            errors
                .iter()
                .any(|e| e.contains("field 'Person.age' expected int")),
            "errors were: {:?}",
            errors
        );
    }

    #[test]
    fn infers_variable_type_from_int_literal() {
        let source = r#"
func test() {
    let x = 42;
}
"#;

        let mut program = BpParse::data(source);
        let result = type_check_program(&mut program);
        assert!(result.is_ok(), "type inference should allow untyped int variable, got: {:?}", result.err());
    }

    #[test]
    fn infers_variable_type_from_string_literal() {
        let source = r#"
func test() {
    let name = "alice";
}
"#;

        let mut program = BpParse::data(source);
        let result = type_check_program(&mut program);
        assert!(result.is_ok(), "type inference should allow untyped string variable, got: {:?}", result.err());
    }

    #[test]
    fn infers_variable_type_from_list_literal() {
        let source = r#"
func test() {
    let nums = [1, 2, 3];
}
"#;

        let mut program = BpParse::data(source);
        let result = type_check_program(&mut program);
        assert!(result.is_ok(), "type inference should allow untyped list variable, got: {:?}", result.err());
    }
}
