use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum Ast {
    Lt(i32),           // <
    Gt(i32),           // >
    Add(i32),          // +
    Sub(i32),          // -
    Dot,               // .
    Comma,             // ,
    Bracket(Vec<Ast>)  // []
}

fn stack(ast: Ast, parsers: &[Ast]) -> Vec<Ast> {
    let mut vec = vec![ast];
    vec.extend_from_slice(&parsers);
    vec
}

macro_rules! opti_expr {
    ($name:ident, $expression:expr) => {
        {
            let mut i: i32 = 0;
            for val in $expression.iter() {
                match val {
                    Token::$name => i += 1,
                    _ => break
                }
            }

            stack(Ast::$name(i + 1), &parse(&$expression[(i as usize)..]))
        }
    };
}

pub fn parse(tokens: &[Token]) -> Vec<Ast> {
    match tokens {
        [] => Vec::new(),
        [Token::Lt] => vec![Ast::Lt(1)],
        [Token::Gt] => vec![Ast::Gt(1)],
        [Token::Add] => vec![Ast::Add(1)],
        [Token::Sub] => vec![Ast::Sub(1)],
        [Token::Dot] => vec![Ast::Dot],
        [Token::Comma] => vec![Ast::Comma],
        [Token::OpenBracket] => panic!("Excessive Opening Bracket"),
        [Token::CloseBracket] => panic!("Unexpected Closing Bracket"),

        [Token::Lt, expr @ ..] => opti_expr!(Lt, expr),
        [Token::Gt, expr @ ..] => opti_expr!(Gt, expr),
        [Token::Add, expr @ ..] => opti_expr!(Add, expr),
        [Token::Sub, expr @ ..] => opti_expr!(Sub, expr),
        [Token::Dot, expr @ ..] => stack(Ast::Dot, &parse(&expr)),
        [Token::Comma, expr @ ..] => stack(Ast::Comma, &parse(&expr)),
        [Token::OpenBracket, expr @ ..] => {

            let mut j: usize = 0;
            let mut result: Option<usize> = None;
            for (i, val) in expr.iter().enumerate() {
                match val {
                    Token::OpenBracket => {
                        j += 1;
                    },
                    Token::CloseBracket => {
                        if j == 0 {
                            result = Some(i);
                            break;
                        } else {
                            j -= 1;
                        }
                    },
                    _ => continue
                }
            }

            let idx = result.expect("Excessive Opening Bracket");

            stack(Ast::Bracket(parse(&expr[..idx])), &parse(&expr[idx + 1..]))
        }
        [Token::CloseBracket, ..] => panic!("Unexpected Closing Bracket"),
    }
}
