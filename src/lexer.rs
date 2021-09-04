#[derive(Debug, Clone)]
pub enum Token {
    Lt,           // <
    Gt,           // >
    Add,          // +
    Sub,          // -
    Dot,          // .
    Comma,        // ,
    OpenBracket,  // [
    CloseBracket  // ]
}

pub fn tokenize(inputs: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    for i in inputs.chars() {
        match i {
            '<' => tokens.push(Token::Lt),
            '>' => tokens.push(Token::Gt),
            '+' => tokens.push(Token::Add),
            '-' => tokens.push(Token::Sub),
            '.' => tokens.push(Token::Dot),
            ',' => tokens.push(Token::Comma),
            '[' => tokens.push(Token::OpenBracket),
            ']' => tokens.push(Token::CloseBracket),
            _ => {}
        }
    }

    tokens
}
