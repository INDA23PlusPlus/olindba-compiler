mod lexer;
pub use crate::lexer::*;

fn main() {
    let mut lexer = Lexer::new(
"if test == 14 {
            test = 16;
        }
        else {
            test = 14;
        }
        while test >= 0 {
            test /= 2;
        }"
    );
    loop {
        match lexer.next_token() {
            Ok(token) => {
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
}
