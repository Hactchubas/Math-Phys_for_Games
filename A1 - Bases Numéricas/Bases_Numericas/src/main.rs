// use std::io::{self, Write};

mod structs;
mod tests;

// fn main() {
//     loop {
//         // Solicita ao usuário para digitar uma entrada
//         print!("Digite um comando (ou 'quit' para sair): ");
//         io::stdout().flush().unwrap();  // Garantir que a mensagem seja impressa imediatamente

//         // Cria uma variável para armazenar a entrada
//         let mut input = String::new();

//         // Lê a entrada do usuário
//         io::stdin().read_line(&mut input).unwrap();

//         // Remove espaços extras e o caractere de nova linha (\n)
//         let input = input.trim();

//         // Verifica se a entrada é "quit" para sair do loop
//         if input == "quit" {
//             println!("Saindo...");
//             break;  // Sai do loop
//         }

//         // Se o usuário não digitar "quit", a aplicação continua o loop
//         println!("Você digitou: {}", input);
//     }
// }

// fn handle_input(input : &str) -> String {
//     let parts: Vec<&str> = input.split_whitespace().collect();

//     for part in parts {
//         match part {
//             ""
//         }
//     }

//     "1".to_string()
// }

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
