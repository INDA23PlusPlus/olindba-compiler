mod lexer;
mod ast;

pub use crate::lexer::*;
pub use crate::ast::*;

fn main() {
    let mut lexer = Lexer::new(
"
if a == b {
    if a == b {
        a = 1;
    }
    a = 2;
}"
    );
    let mut tokens = vec![];
    loop {
        match lexer.next_token() {
            Ok(token) => {
                tokens.push(token.clone());
                if token.ty == TokenType::EOF {
                    break;
                }
                else {
                    println!("{:?}", token);
                }
            },
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }
    let mut ast = Ast::new(tokens);
    let ast_sequence = ast.generate_ast();
    match ast_sequence {
        Ok(sequence) => {
            for ting in sequence {
                println!("{:?}", ting);
            }
        },
        Err(err) => {
            println!("{:?}", err);
        }
    }
}
