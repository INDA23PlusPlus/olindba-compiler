use crate::ast::*;

pub fn generate_code(ast: Ast) -> String {
    let mut code = "#include<iostream>\n".to_string();
    if !ast.seen_variables.is_empty() {
        code += "int ";
        for variable in ast.seen_variables {
            code += (variable + ",").as_str();
        }
        code = code[..code.len() - 1].to_string() + ";";
    }
    code += "int main(){";
    for node in ast.sequence {
        code += parse_node(node).as_str();
    }
    code + "}"
}

fn parse_node(node: Node) -> String {
    let mut code = String::new();
    match node {
        Node::If { condition, body } => {
            code += add_if(condition, body).as_str();
        },
        Node::IfElse { condition, body, else_body } => {
            code += add_if(condition, body).as_str();
            code += "else";
            code += add_body(else_body).as_str();
        },
        Node::Loop { loop_count, body } => {
            code += format!("for(int i=0;i<{loop_count};i++)", loop_count = loop_count).as_str();
            code += add_body(body).as_str();
        },
        Node::Print( value ) => {
            code += format!("std::cout<<{value}<<std::endl;", value = value).as_str();
        },
        Node::VariableAssignment { value, identifier } => {
            code += format!("{variable}={value};", variable = identifier, value = parse_expression(value)).as_str();
        },
        Node::While { condition, body } => {
            code += format!("while({expression})", expression = parse_expression(condition)).as_str();
            code += add_body(body).as_str();
        }
    }
    code
}

fn add_if(condition: Expression, body: Vec<Node>) -> String {
    let mut code = String::new();
    code += format!("if({expression})", expression = parse_expression(condition)).as_str();
    code += add_body(body).as_str();
    code
}

fn add_body(body: Vec<Node>) -> String {
    let mut code = String::new();
    code += "{";
    for node in body {
        code += parse_node(node).as_str();
    }
    code += "}";
    code
}

fn parse_expression(expression: Expression) -> String {
    match expression {
        Expression::Operation { left, right, raw_operator } => {
            return format!(
                "{left}{operator}{right}", 
                left = parse_expression(*left),
                right = parse_expression(*right),
                operator = raw_operator
            );
        },
        Expression::Value(value) => {
            return value;
        }
    }
}