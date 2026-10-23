#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Number(f64),
    Symbol(String),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, u32),
}

impl Expr {
    pub fn symbol(name: impl Into<String>) -> Self { Self::Symbol(name.into()) }

    pub fn add(self, rhs: Expr) -> Self { Self::Add(Box::new(self), Box::new(rhs)) }
    pub fn mul(self, rhs: Expr) -> Self { Self::Mul(Box::new(self), Box::new(rhs)) }
    pub fn pow(self, exponent: u32) -> Self { Self::Pow(Box::new(self), exponent) }

    pub fn differentiate(&self, variable: &str) -> Expr {
        match self {
            Expr::Number(_) => Expr::Number(0.0),
            Expr::Symbol(name) => Expr::Number(if name == variable { 1.0 } else { 0.0 }),
            Expr::Add(a, b) => a.differentiate(variable).add(b.differentiate(variable)),
            Expr::Mul(a, b) => a.differentiate(variable).mul((**b).clone()).add((**a).clone().mul(b.differentiate(variable))),
            Expr::Pow(base, n) if *n > 0 => Expr::Number(*n as f64).mul(base.clone().pow(n - 1)).mul(base.differentiate(variable)),
            Expr::Pow(_, _) => Expr::Number(0.0),
        }
    }
}
