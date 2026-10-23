#[derive(Clone, Debug, PartialEq)]
pub enum Token { Ident(String), Number(f64), Let, If, Else, End, Return, Plus, Minus, Star, Slash, Eq, EqEq, LParen, RParen, Comma, Newline }

pub fn lex(source: &str) -> Result<Vec<Token>, String> {
    let mut out = Vec::new();
    let mut chars = source.chars().peekable();
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() { chars.next(); continue; }
        if c.is_ascii_digit() || c == '.' {
            let mut s = String::new();
            while chars.peek().is_some_and(|x| x.is_ascii_digit() || *x == '.') { s.push(chars.next().unwrap()); }
            out.push(Token::Number(s.parse().map_err(|_| format!("invalid number: {s}"))?));
        } else if c.is_ascii_alphabetic() || c == '_' {
            let mut s = String::new();
            while chars.peek().is_some_and(|x| x.is_ascii_alphanumeric() || *x == '_') { s.push(chars.next().unwrap()); }
            out.push(match s.as_str() { "let" => Token::Let, "if" => Token::If, "else" => Token::Else, "end" => Token::End, "return" => Token::Return, _ => Token::Ident(s) });
        } else {
            chars.next();
            out.push(match c { '+' => Token::Plus, '-' => Token::Minus, '*' => Token::Star, '/' => Token::Slash, '(' => Token::LParen, ')' => Token::RParen, ',' => Token::Comma, '=' => if chars.peek() == Some(&'=') { chars.next(); Token::EqEq } else { Token::Eq }, '\n' => Token::Newline, _ => return Err(format!("unexpected character: {c}")) });
        }
    }
    Ok(out)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr { Number(f64), Name(String), Binary { op: Token, left: Box<Expr>, right: Box<Expr> } }

#[derive(Clone, Debug, PartialEq)]
pub enum Statement { Let(String, Expr), Return(Expr), If { condition: Expr, then_branch: Vec<Statement>, else_branch: Vec<Statement> } }

pub fn parse_expression(source: &str) -> Result<Expr, String> {
    let tokens = lex(source)?;
    if tokens.len() == 1 { match &tokens[0] { Token::Number(n) => return Ok(Expr::Number(*n)), Token::Ident(s) => return Ok(Expr::Name(s.clone())), _ => {} } }
    Err("expression parser currently accepts a single literal or identifier".into())
}
