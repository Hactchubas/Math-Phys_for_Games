// use std::io::{self, Write};



mod structs;
mod tests;



fn inverte_binario(binario: &str) -> String {
    binario.chars().rev().collect()
}

fn complemento_de_dois(binario: &str) -> String {
    let mut resultado = String::new();
    for c in binario.chars() {
        resultado.push(if c == '0' { '1' } else { '0' });
    }
    soma_um(&resultado)
}

fn soma_um(binario: &str) -> String {
    let mut resultado = String::new();
    let mut carry = true;
    for c in binario.chars().rev() {
        match (c, carry) {
            ('0', false) => resultado.push('0'),
            ('0', true) => {
                resultado.push('1');
                carry = false;
            }
            ('1', false) => resultado.push('1'),
            ('1', true) => resultado.push('0'),
            (_, _)=> {}
        }
    }
    if carry {
        resultado.push('1');
    }
    inverte_binario(&resultado)
}

fn soma_binarios(a: &str, b: &str) -> String {
    let mut resultado = String::new();
    let mut carry = false;
    for (a, b) in a.chars().rev().zip(b.chars().rev()) {
        match (a, b, carry) {
            ('0', '0', false) => resultado.push('0'),
            ('0', '0', true) => {
                resultado.push('1');
                carry = false;
            }
            ('0', '1', false) => resultado.push('1'),
            ('0', '1', true) => resultado.push('0'),
            ('1', '0', false) => resultado.push('1'),
            ('1', '0', true) => resultado.push('0'),
            ('1', '1', false) => {
                resultado.push('0');
                carry = true;
            }
            ('1', '1', true) => resultado.push('1'),
            (_,_,_) => {}
        }
    }
    if carry {
        resultado.push('1');
    }
    inverte_binario(&resultado)
}

fn subtrai_binarios(a: &str, b: &str) -> String {
    let b_complemento = complemento_de_dois(b);
    let mut resultado = soma_binarios(a, &b_complemento);
    if resultado.starts_with('0') {
        resultado = resultado.trim_start_matches('0').to_string();
    }
    resultado
}

fn main() {
    let a = "10010101001000110";
    let b = "1110101010100";
    println!("Soma: {}", soma_binarios(a, b));
    println!("Subtração: {}", subtrai_binarios(a, b));
}
