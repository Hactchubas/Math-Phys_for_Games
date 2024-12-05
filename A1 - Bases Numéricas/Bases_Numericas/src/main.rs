// use std::io::{self, Write};

mod structs;
mod tests;

use structs::binary::{Binary, BinaryFactory};

fn evaluate_expression(expression: &str) -> Result<String, &str> {

    let mut result: Binary = BinaryFactory::create("0").unwrap_or_default();
    let mut current_operator: char = '+';
    let mut is_first = true; 

    for token in expression.split_whitespace() {
        // println!("{}, {}" ,token ,result.get_binary());
        if let Some(binary) = BinaryFactory::create(token) {
            let b= binary.clone();
            if is_first {
                result = b; 
                is_first = false;
            } else {
                result = match current_operator {
                    '+' => result + b,
                    '-' => result - b,
                    '*' => result * b,
                    _ => result,
                };
            }
        } else if token == "+" || token == "-" || token == "*" {
            current_operator = token.chars().next().unwrap();
        } else {
            return Err("Not a valid expression");
        }
        
    }

    Ok(result.get_binary().to_string())
}

fn main() {
    let expression = "1 + 100 - 101";
    match evaluate_expression(expression) {
        Ok(s) => println!("Resultado: {}", s),
        Err(s) => println!("{}", s.to_string()),
    };
}
