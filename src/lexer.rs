#[derive(PartialEq, Eq)]
pub struct Punctuation {
    raw: String,
    kind: PunctutationKind
}

#[derive(PartialEq, Eq, Debug)]
pub enum PunctutationKind {
    Open(usize),
    Close(usize),
    Seperator
}

#[derive(PartialEq, Eq, Debug)]
pub enum TokenType {
    Punctuation{raw: String, kind: PunctutationKind},
    Operator(String),
    Identifier(String),
    Number(String),
    EOF
}
#[derive(PartialEq, Eq, Debug)]
pub struct Token {
    pub ty: TokenType,
    collumn: usize,
    line: usize
}

#[derive(Debug)]
pub struct LexerErr {
    pub kind: LexerErrKind,
    pub collumn: usize,
    pub line: usize
}

#[derive(Debug)]
pub enum LexerErrKind {
    UnevenParentheses,
    IncorrectNumber,
    IncorrectIdentifier,
}

pub struct Lexer<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    cur_line: usize,
    cur_collumn: usize,
    cur_curly: usize,
    cur_paren: usize
}

impl<'a> Lexer<'a> {
    pub fn new(chars: &'a str) -> Lexer<'a> {
        Lexer {
            chars: chars.chars().peekable(),
            cur_line: 0,
            cur_collumn: 0,
            cur_curly: 0,
            cur_paren: 0
        }
    }

    fn is_identifier(c: char) -> bool { 
        match c {
            'A'..='Z' | 'a'..='z' | '_' => true, 
            _ => false 
        }
    }

    fn is_punctuation(c: char)      -> bool { "(){},;\"".contains(c) }
    fn is_seperator(c: char)        -> bool { ",;\"".contains(c) }
    fn is_part_of_operator(c: char) -> bool { "+=-*/><!&|".contains(c) }


    fn parse_identifier(&mut self) -> Token { 
        let mut raw = "".to_string();
        loop {
            if let Some(c) = self.chars.peek() {
                if Lexer::is_identifier(*c) || (raw.len() > 0 && c.is_numeric()) {
                    raw.push(*c);
                    self.chars.next();
                    self.cur_collumn += 1;
                }
                else { break; }
            }
            else { break; }
        }
        return Token {
            ty: TokenType::Identifier(raw),
            collumn: self.cur_collumn,
            line: self.cur_line
        }
    }

    fn parse_number(&mut self) -> Result<Token, LexerErr> {
        let mut raw = "".to_string();
        loop {
            if let Some(c) = self.chars.peek() {
                if c.is_numeric() {
                    raw.push(*c);
                    self.chars.next();
                    self.cur_collumn += 1;
                }
                else if Lexer::is_identifier(*c) {
                    return Err(LexerErr {
                        kind: LexerErrKind::IncorrectNumber,
                        collumn: self.cur_collumn,
                        line: self.cur_line
                    })
                }
                else { break; }
            }
            else { break; }
        }
        return Ok(Token {
            ty: TokenType::Number(raw),
            collumn: self.cur_collumn,
            line: self.cur_line
        })
    }

    fn parse_paren(&mut self) -> Result<Token, LexerErr> {
        let c = self.chars.next().expect("Always exists").to_string();
        match c.as_str() {
            "(" | "{" => {
                let token = Token {
                    ty: TokenType::Punctuation { 
                        raw: c.clone(), 
                        kind: PunctutationKind::Open(match c.as_str() {
                            "(" => self.cur_paren,
                            "{" => self.cur_curly,
                            _ => 0
                        }) 
                    },
                    collumn: self.cur_collumn,
                    line: self.cur_line
                };
                match c.as_str() {
                    "(" => { self.cur_paren += 1; },
                    "{" => { self.cur_curly += 1; },
                    _ => {}
                }
                return Ok(token);
            },

            ")" | "}" => {
                let current = match c.as_str() {
                    ")" => self.cur_paren,
                    "}" => self.cur_curly,
                    _ => 0
                };
                if current == 0 {
                    return Err(LexerErr{
                        kind: LexerErrKind::UnevenParentheses,
                        collumn: self.cur_collumn,
                        line: self.cur_line
                    })
                }
                else {
                    match c.as_str() {
                        ")" => { self.cur_paren -= 1; },
                        "}" => { self.cur_curly -= 1; },
                        _ => {}
                    }
                    return Ok(Token {
                        ty: TokenType::Punctuation { 
                            raw: c.clone(), 
                            kind: PunctutationKind::Close(current - 1) 
                        },
                        collumn: self.cur_collumn,
                        line: self.cur_line
                    });
                }
            },
            _ => {
                panic!("Will never be reached");
            }
        }
    }

    pub fn next_token(&mut self) -> Result<Token, LexerErr> {

        loop {
            if let Some(c) = self.chars.peek() {

                self.cur_collumn += 1;
                if c.is_whitespace() {
                    if *c == '\n' {
                        self.cur_line += 1;
                        self.cur_collumn = 0;
                    }
                    self.chars.next();
                    continue;
                }

                if Lexer::is_identifier(*c) {
                    return Ok(self.parse_identifier());
                }

                else if c.is_numeric() {
                    return self.parse_number();
                }

                else if Lexer::is_punctuation(*c) {    
    
                    if Lexer::is_seperator(*c) {
                        return Ok(Token {
                            ty: TokenType::Punctuation { 
                                raw: self.chars.next().expect("Always exists").to_string(), 
                                kind: PunctutationKind::Seperator 
                            },
                            collumn: self.cur_collumn,
                            line: self.cur_line
                        });
                    }
                    return self.parse_paren();
                }

                else if Lexer::is_part_of_operator(*c) {
                    let mut operator = self.chars.next().expect("Always exists").to_string();
                    if let Some(nxt) = self.chars.peek() {
                        if Lexer::is_part_of_operator(*nxt) {
                            operator.push(*nxt);
                            self.chars.next();
                            self.cur_collumn += 1;
                        }
                    }
                    return Ok(Token {
                        ty: TokenType::Operator(operator),
                        collumn: self.cur_collumn,
                        line: self.cur_line
                    })
                }
            }

            if self.cur_curly > 0 || self.cur_paren > 0 {
                self.cur_curly = 0;
                self.cur_paren = 0;
                return Err(LexerErr {
                    kind: LexerErrKind::UnevenParentheses,
                    collumn: self.cur_collumn,
                    line: self.cur_line
                })
            }
            return Ok(Token {
                ty: TokenType::EOF,
                collumn: self.cur_collumn,
                line: self.cur_line
            });
        }
    }
}