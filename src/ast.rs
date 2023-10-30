#![allow(dead_code)]
use std::collections::HashSet;
use crate::lexer::*;

#[derive(Debug)]
pub struct AstErr {
    collumn: usize,
    line: usize
}

#[derive(Debug)]
pub enum Expression {
    Operation {
        left: Box<Expression>,
        right: Box<Expression>,
        raw_operator: String
    },
    Value(String)
}

#[derive(Debug)]
pub enum Node {
    If {
        condition: Expression,
        body: Vec<Node>
    },
    IfElse {
        condition:Expression,
        body: Vec<Node>,
        else_body: Vec<Node>,
    },
    Loop {
        loop_count: usize,
        body: Vec<Node>
    },
    While {
        condition: Expression,
        body: Vec<Node>
    },
    VariableAssignment {
        identifier: String,
        value: Expression,
        declaration: bool
    },
    Print(String)
}

pub struct Ast {
    tokens: Vec<Token>,
    cur: usize,
    seen_variables: HashSet<String>
}

impl Ast {
    pub fn new(tokens: Vec<Token>) -> Ast {
        Ast { 
            tokens: tokens,
            cur: 0,
            seen_variables: HashSet::new()
        }
    }   

    pub fn generate_ast(&mut self) -> Result<Vec<Node>, AstErr> {
        let mut sequence = vec![];
        
        loop {
            if self.tokens[self.cur].ty == TokenType::EOF {
                break;
            }
            let node = self.parse_line();
            match node {
                Ok(node) => {
                    sequence.push(node);
                },
                Err(err) => {
                    return Err(err);
                }
            }
        }
        Ok(sequence)
    }

    fn parse_line(&mut self) -> Result<Node, AstErr> {
        let token = self.tokens[self.cur].clone();
        self.cur += 1;

        match token.ty {
            TokenType::Identifier => {

                if is_keyword(&token.raw) {
                    match token.raw.as_str() {
                        "if" => {
                            let condition = self.parse_expression();
                            let body = self.parse_body();
                            match body {
                                Ok(body) => {
                                    self.cur += 1;
                                    if self.tokens[self.cur].raw == "else" {
                                        self.cur += 2;
                                        let else_body = self.parse_body();
                                        match else_body {
                                            Ok(else_body) => {
                                                self.cur += 1;
                                                return Ok(Node::IfElse { 
                                                    condition: condition, 
                                                    body: body, 
                                                    else_body: else_body 
                                                })
                                            },
                                            Err(err) => { return Err(err); }
                                        }
                                    }
                                    else {
                                        return Ok(Node::If {
                                            condition: condition, 
                                            body: body 
                                        })
                                    }
                                },
                                Err(err) => { return Err(err); }
                            }
                        },
                        "while" => {
                            let condition = self.parse_expression();
                            let body = self.parse_body();
                            self.cur += 1;
                            match body {
                                Ok(body) => {
                                    return Ok(Node::While { 
                                        condition: condition, 
                                        body: body 
                                    })
                                },
                                Err(err) => { return Err(err); }
                            }
                        },
                        "loop" => { 
                            if self.tokens[self.cur].ty != TokenType::Number { 
                                return Err(AstErr { 
                                    collumn: self.tokens[self.cur].collumn, 
                                    line: self.tokens[self.cur].line,
                                }) 
                            }
                            let loop_count = self.tokens[self.cur].raw.parse::<usize>().unwrap();
                            self.cur += 1;
                            let body = self.parse_body();
                            match body {
                                Ok(body) => {
                                    return Ok(Node::Loop { 
                                        loop_count: loop_count, 
                                        body: body 
                                    })
                                },
                                Err(err) => { return Err(err); }
                            }
                        },
                        "print" => {
                            self.cur += 3;
                            return Ok(Node::Print(self.tokens[self.cur - 2].raw.clone()));
                        },
                        _ => {
                            return Err(AstErr { 
                                collumn: self.tokens[self.cur].collumn, 
                                line: self.tokens[self.cur].line
                            })
                        }
                    }
                }
                else {
                    self.cur += 1;
                    let expression = self.parse_expression();
                    let node = Node::VariableAssignment { 
                        identifier: token.raw.clone(), 
                        value: expression, 
                        declaration: !self.seen_variables.contains(&token.raw)
                    };
                    self.seen_variables.insert(token.raw);
                    return Ok(node);
                }
            },
            _ => {
                return Err(AstErr {
                    collumn: token.collumn,
                    line: token.line
                });
            }
        }
    }

    fn parse_expression(&mut self) -> Expression {
        self.cur += 1;
        if self.tokens[self.cur].raw == ";" || self.tokens[self.cur].raw == "{"  {
            self.cur += 1;
            return Expression::Value(self.tokens[self.cur - 2].raw.clone());
        }
        if self.tokens[self.cur].raw == "{" {
            return Expression::Value(self.tokens[self.cur - 1].raw.clone());
        }
        self.cur += 1;
        let raw_operator = self.tokens[self.cur - 1].raw.clone();
        return Expression::Operation { 
            left: Box::new(Expression::Value(self.tokens[self.cur - 2].raw.clone())), 
            right: Box::new(self.parse_expression()),
            raw_operator: raw_operator
        }
    }

    fn parse_body(&mut self) -> Result<Vec<Node>, AstErr> {
        let mut body = vec![];
        while self.tokens[self.cur].ty != TokenType::EOF && self.tokens[self.cur].raw != "}" {
            let node = self.parse_line();
            match node {
                Ok(node) => {
                    body.push(node);
                },
                Err(err) => { return Err(err); }
            }
        }
        Ok(body)
    }
}

fn is_keyword(raw: &String) -> bool {
    match raw.as_str() {
        "while" | "loop" | "if" | "else" | "print" => true,
        _ => false
    }
}