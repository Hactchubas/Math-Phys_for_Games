// use std::io::{self, Write};

mod structs;
mod tests;

use std::io::{self, Write};

use structs::binary::{Binary, BinaryFactory};

fn evaluate_expression(expression: &str) -> Result<String, &str> {
    let mut result: Binary = BinaryFactory::create("0").unwrap_or_default();
    let mut current_operator: char = '+';
    let mut is_first = true;

    let mut open_bracket = false;
    let mut bracket_value: Binary;

    for (index, token) in expression.split_whitespace().enumerate() {
        if open_bracket {
            continue;
        }
        if let Some(binary) = BinaryFactory::create(token) {
            
            let b = binary.clone();
            if is_first {
                result = b;
                is_first = false;
            } else {
                result = match current_operator {
                    '+' => result + b,
                    '-' => result - b,
                    '*' => result * b,
                    '&' => result & b,
                    '|' => result | b,
                    '^' => result ^ b,
                    _ => result,
                };
            }
        } else if token == "+"
            || token == "-"
            || token == "*"
            || token == "&"
            || token == "|"
            || token == "^"
        {
            current_operator = token.chars().next().unwrap();
        } else if token == "(" {
            open_bracket = true;
            match evaluate_expression(expression.split_at(index+1).1) {
                Ok(b) => {
                    if let Some(binary) = BinaryFactory::create(&b) {
                        bracket_value = binary;
                        result = match current_operator {
                            '+' => result + bracket_value,
                            '-' => result - bracket_value,
                            '*' => result * bracket_value,
                            '&' => result & bracket_value,
                            '|' => result | bracket_value,
                            '^' => result ^ bracket_value,
                            _ => result,
                        };
                    } else {
                        return Err("Expressão inválida");
                    }
                } 
                Err(s) => return Err(s)
            }
        } else if token == ")" {
            return Ok(result.get_binary().to_string())        
        } else {
            return Err("Expressão inválida");
        }
    }

    Ok(result.get_binary().to_string())
}

fn handle_input() {
    print!(
        "((( Escreva 'sair' para sair da aplicação )))
        Olá, sou sua calculadora de binário
        Digite uma expressão comando para ser calculada, exemplo:
        \t\t'101 + 1' \tou\t '101 - 10 * 11' \tou\t '101 - ( 10 * 11 )'
        Importante:
        -> Digite os números, expressões e parenteses separados por espaços ' '
        -> Espressões são sempre avaliadas da esquerda para direita em ordem, independente das operações:
        Exemplo: 1000 - 100 * 11 => 100 * 11 => 1100 (Output: 01100)
        As operações permitidas:
        \tAND Bit a Bit:    '&'
        \tOR Bit a Bit:     '|'
        \tXOR Bit a Bit:    '^'
        \tAdição:           '+'
        \tSubtração:        '-'
        \tMultiplicação:    '*'
        \nDivirta-se:\n"
    );
    loop {
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input == "sair" {
            println!("\nSaindo...");
            break;
        }
        println!("Você digitou: {}", input);
        match evaluate_expression(input) {
            Ok(s) => println!("Resultado: {}\n", s),
            Err(s) => println!("{}", s.to_string()),
        };
    }
}

fn main() {
    handle_input();
}
