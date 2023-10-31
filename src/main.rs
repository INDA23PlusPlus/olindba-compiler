mod lexer;
mod ast;
mod gen;

pub use crate::lexer::*;
pub use crate::ast::*;
pub use crate::gen::*;

use std::fs;

fn main() {
    let inp_code = fs::read_to_string("test/test.txt").unwrap();
    let mut lexer = Lexer::new(inp_code.as_str());
    let mut tokens = vec![];
    loop {
        match lexer.next_token() {
            Ok(token) => {
                tokens.push(token.clone());
                if token.ty == TokenType::EOF { break; }
                else { println!("{:?}", token); }
            },
            Err(err) => { println!("{:?}", err); }
        }
    }
    let mut ast = Ast::new(tokens);
    let ast_err = ast.generate_ast();
    if let Some(err) = ast_err {
        println!("{:?}", err);
    }
    else {
        for node in ast.sequence.clone() {
            println!("{:?}", node);
        }
    }

    let code_out = generate_code(ast.clone());
    let _ = fs::write("test/out.cpp", code_out);
}
