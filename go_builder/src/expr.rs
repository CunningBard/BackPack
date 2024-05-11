
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Var(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

impl Expr {
    /// Simplify the expression by evaluating constant subexpressions.
    /// 
    /// When it can't simplify the expression, it returns a clone of the original expression.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use go_builder::expr::Expr;
    /// 
    /// let expr = Expr::Add(Box::new(
    ///     Expr::Int(1)), 
    ///     Box::new(Expr::Int(2)));
    /// 
    /// assert_eq!(expr.simplify(), Expr::Int(3));
    /// 
    /// let expr = Expr::Add(
    ///     Box::new(Expr::Int(1)), 
    ///     Box::new(Expr::Float(2.0)));
    /// 
    /// assert_eq!(expr.simplify(), Expr::Add(Box::new(Expr::Int(1)), Box::new(Expr::Float(2.0))));
    /// 
    /// let expr = Expr::Add(
    ///     Box::new(Expr::Int(1)), 
    ///     Box::new(Expr::Mul(
    ///         Box::new(Expr::Int(2)), 
    ///         Box::new(Expr::Int(3)))));
    /// 
    /// assert_eq!(expr.simplify(), Expr::Int(7));
    /// 
    /// let expr = Expr::Add(
    ///     Box::new(Expr::Var("x".to_string())), 
    ///     Box::new(Expr::Mul(
    ///         Box::new(Expr::Int(2)), 
    ///         Box::new(Expr::Int(3)))));
    /// 
    /// assert_eq!(expr.simplify(), Expr::Add(Box::new(Expr::Var("x".to_string())), Box::new(Expr::Int(6))));
    /// ```
    pub fn simplify(&self) -> Expr {
        macro_rules! simplify {
            ($variant:ident, $op:tt, $left:expr, $right:expr) => {
                match ($left.simplify(), $right.simplify()) {
                    (Expr::Int(l), Expr::Int(r)) => Expr::Int(l $op r),
                    (Expr::Float(l), Expr::Float(r)) => Expr::Float(l $op r),
                    (left, right) => Expr::$variant(Box::new(left), Box::new(right)),
                }
            };
        }
        match self {
            Expr::Add(left, right) => {
                simplify!(Add, +, left, right)
            }
            Expr::Sub(left, right) => {
                simplify!(Sub, -, left, right)
            }
            Expr::Mul(left, right) => {
                simplify!(Mul, *, left, right)
            }
            Expr::Div(left, right) => {
                simplify!(Div, /, left, right)
            }
            e => e.clone(),
        }
    }
}