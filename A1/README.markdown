### Kauã Souza Maia - 536732 

## Atividade de Implementação – Bases Numéricas

### Enunciado 

- Números binários serão representados como strings no
seu programa. Ex: “1010” significa 10 em decimal
- Escreva um programa que receba um número binário
como string e realize as seguintes operações:
  - Validação: Checagem se a string é realmente um número binário
válido. Erros devem ser reportado para números inválidos, porém
sem encerrar seu programa.
  - Sucessor: dado um número binário “x”, retorne a string (número
binário) que representa “x” + “1”.
  - Inverso: dado um número binário “x”, retorne o número que seja o
inverso bit a bit de cada dígito de “x”.
  - Complemento de 2: dado um número binário positivo “x”, retorne
o número representa “-x”.

- Ainda assumindo a representação via strings, implemente
as seguintes operações
  - AND bit a bit: calcule “x” & “y” considerando todos os bits dos
números
-   OR bit a bit: calcule “x” | “y” considerando todos os bits dos números 
-   XOR bit a bit: calcule “x” ^ “y” considerando todos os bits dos números 
-   Adição: calcule “x” + “y” 
-   Subtração: calcule “x” - “y” 
-   Multiplicação: calcule “x” * “y” 
-   Divisão: calcule “x” / “y” 
-   Resto da Divisão: calcule “x” % “y” 


## Implementação

### Binário de forma geral
```rust
#[derive(Debug, Clone)]
pub struct Binary {
    value: String,
    signal: char,
    cached_value: String,
}

impl Binary {
    pub fn new(value: String, signal: char) -> Binary {
        let cached_value = match signal {
            '1' => format!("-{}", value),
            _ => format!("{}", value),
        };
        Binary {
            value,
            signal,
            cached_value,
        }
    }

    pub fn get_binary(&self) -> &str {
        &self.cached_value
        
    }

    /*
    Return both binaries in the comparison 
    with the same number of digits    
    */
    fn pad_compare(&self, other: &Self) -> (Binary, Binary) {
        let max_len = self.value.len().max(other.value.len());

        let mut x_padded = self.clone();
        let mut y_padded = other.clone();
        x_padded.value = format!("{:0>width$}", x_padded.value, width = max_len);
        y_padded.value = format!("{:0>width$}", y_padded.value, width = max_len);
        (x_padded, y_padded)
    }

    /*
    Return the same binarie but with the signal inverted   
    */
    fn change_signal(&self) -> Binary {
        let new_signal = match self.signal {
            '1' => '0',
            '0' => '1',
            _ => panic!("Erro de sinal"),
        };
        let new_value = &self.value;
        return Binary::new(new_value.to_string(), new_signal);
    }

    /*
    Compares two binaries by its {value} only  
    */
    fn greater_value(&self, other: &Self) -> (Binary, Binary, bool) {
        match self.clone().value > other.clone().value {
            true => (self.clone(), other.clone(), false),
            false if self.clone().value < other.clone().value => {
                (other.clone(), self.clone(), false)
            }
            _ => (self.clone(), other.clone(), true),
        }
    }

    /*
    Treats the binary if equals to zero so it does not
    return '-0' or weirder  
    */
    fn treat_zero(&self) -> Binary {
        if self.not_zero() {
            self.clone()
        } else {
            Binary::new("0".to_string(), '0')
        }
    }

    /*
    Check if :Binary it's not zero
    */
    fn not_zero(&self) -> bool {
        !self.clone().value.chars().all(|bit| bit == '0')
    }

    /*
    Remove unecesseary digits '0'  
    */
    fn trim(&self) -> Binary {
        if self.not_zero() {
            let mut trimmed = self.clone();
            trimmed.value = trimmed.value.trim_start_matches('0').to_string();
            trimmed
        } else {
            self.treat_zero()
        }
    }
}
impl Default for Binary {
    fn default() -> Self {
        Binary::new("0".to_string(), '0')
    }
}

impl Ord for Binary {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cached_value
            .cmp(&other.cached_value)
            .then(self.signal.cmp(&other.signal))
    }
}
impl PartialOrd for Binary {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Binary {
    fn eq(&self, other: &Self) -> bool {
        self.trim().get_binary() == other.trim().get_binary()
    }
}
impl Eq for Binary {}
```

### Validação
- Checagem se a string é realmente um número binário válido. Erros devem ser reportado para números inválidos, porém sem encerrar seu programa.

```rust
pub struct BinaryFactory;
impl BinaryFactory {
    pub fn create(binary: &str) -> Option<Binary> {
        let mut signal: &str;
        let value: String;

        let aux: &str;
        (signal, aux) = binary.split_at(1);

        match signal {
            "-" => {
                signal = "1";
                value = aux.to_string();
            }
            _ => {
                signal = "0";
                value = binary.to_string();
            }
        };
        if Self::validate(&value).is_ok() {
            Some(Binary::new(value, signal.chars().next().unwrap()))
        } else {
            None
        }
    }
    fn validate(value: &str) -> Result<(), &str> {
        if value.chars().all(|c| c == '0' || c == '1') && !value.is_empty() {
            Ok(())
        } else {
            Err("Not a valid binary!")
        }
    }
}
```

### Sucessor
- Dado um número binário “x”, retorne a string (número
binário) que representa “x” + “1”.

```rust
impl Binary {
    pub fn next(&self) -> Binary {
        let mut bits: Vec<char> = self.value.chars().collect();
        let mut carry = true;

        for i in (0..bits.len()).rev() {
            if carry {
                match bits[i] {
                    '0' => {
                        bits[i] = '1';
                        carry = false
                    }
                    '1' => bits[i] = '0',
                    _ => panic!("invalid binary Binary"),
                }
            }
        }
        if carry {
            bits.insert(0, '1');
        }
        let result: String = bits.iter().collect();
        Binary::new(result, self.signal)
    }
}
```

### Inverso
- Dado um número binário “x”, retorne o número que seja o
inverso bit a bit de cada dígito de “x”.

```rust
impl Binary {
    pub fn invert(&self) -> Binary {
        let mut bits: Vec<char> = self.value.chars().collect();
        for i in 0..bits.len() {
            match bits[i] {
                '0' => bits[i] = '1',
                '1' => bits[i] = '0',
                _ => panic!("invalid binary Binary"),
            }
        }
        let result: String = bits.iter().collect();
        Binary::new(result, self.signal)
    }
}
```

### Complemento de 2
- Dado um número binário positivo “x”, retorne
o número representa “-x”.

```rust
impl Binary {
    pub fn two_complement(&self) -> Binary {
        if self.value == "0" {
            return Binary::new("0".to_string(), '0');
        }
        let result = self.invert().next().change_signal();
        result
    }
}
```

### AND bit a bit
- Calcule “x” & “y” considerando todos os bits dos
números

```rust
impl BitAnd for Binary {
    type Output = Binary;

    fn bitand(self, other: Self) -> Self::Output {
        let (x, y) = self.pad_compare(&other);

        let result: String = x
            .value
            .chars()
            .zip(y.value.chars())
            .map(|(a, b)| if a == '1' && b == '1' { '1' } else { '0' })
            .collect();

        Binary::new(result, self.signal)
    }
}
```

### OR bit a bit
- Calcule “x” | “y” considerando todos os bits dos números 

```rust
impl BitOr for Binary {
    type Output = Binary;
    fn bitor(self, other: Self) -> Self::Output {
        let (x, y) = self.pad_compare(&other);

        let result: String = x
            .value
            .chars()
            .zip(y.value.chars())
            .map(|(a, b)| if a == '1' || b == '1' { '1' } else { '0' })
            .collect();

        Binary::new(result, self.signal)
    }
}
```

### XOR bit a bit
- Calcule “x” ^ “y” considerando todos os bits dos números 

```rust
impl BitXor for Binary {
    type Output = Binary;
    fn bitxor(self, other: Self) -> Self::Output {
        let (x, y) = self.pad_compare(&other);

        let result: String = x
            .value
            .chars()
            .zip(y.value.chars())
            .map(|(a, b)| if a != b { '1' } else { '0' })
            .collect();

        Binary::new(result, self.signal)
    }
}
```

### Adição
- Calcule “x” + "y"

```rust
impl Binary {
  fn addinng(self, other: &Self) -> Binary {
        let mut result = String::new();
        let (x, y) = Binary::pad_compare(&self, other);
        let mut carry = false;

        for (xb, yb) in x
            .value
            .chars()
            .rev()
            .zip(y.value.chars().rev())
            .map(|(a, b)| {
                (
                    Binary::new(a.to_string(), '0'),
                    Binary::new(b.to_string(), '0'),
                )
            })
        {
            let xor: Binary = xb.clone() ^ yb.clone();
            let and: Binary = xb.clone() & yb.clone();

            if xor.value == "1" {
                if carry {
                    result.push('0');
                } else {
                    result.push('1');
                    carry = false;
                }
            } else {
                if carry {
                    result.push('1');
                } else {
                    result.push('0');
                }
                if and.value == '1'.to_string() {
                    carry = true;
                } else {
                    carry = false;
                }
            }
        }

        if carry != false {
            result.push('1');
        }

        Binary::new(result.chars().rev().collect(), '0')
    }
}
impl Add for Binary {
    type Output = Binary;
    fn add(self, other: Self) -> Self::Output {
        match (self.signal, other.signal) {
            ('0', '0') => {
                let (x, y) = self.pad_compare(&other);
                x.addinng(&y)
            }
            ('0', '1') | ('1', '0') => {
                let (x, y, equal) = self.greater_value(&other);
                if equal {
                    return Binary::new("0".to_string(), '0');
                }
                let (x_padded, y_padded) = x.pad_compare(&y);
                let not_treated_result = x_padded.addinng(&y_padded.two_complement());
                let (_, value) = not_treated_result.value.split_at(1);

                Binary::new(value.to_string(), x.signal)
            }
            ('1', '1') => {
                let (x, y) = self.two_complement().pad_compare(&other.two_complement());
                x.addinng(&y).two_complement()
            }
            _ => panic!(),
        }
        .treat_zero()
    }
}
```

### Subtração
- Calcule “x” - “y”

```rust
impl Sub for Binary {
    type Output = Binary;
    fn sub(self, other: Self) -> Self::Output {
        let (x, y) = self.pad_compare(&other);
        x + y.change_signal()
    }
}
```

### Multiplicação
- Calcule “x” * “y”  

```rust
impl Mul for Binary {
    type Output = Binary;

    fn mul(self, other: Self) -> Self::Output {
        let length = other.value.len() - 1;
        let mut r = other
            .value
            .char_indices()
            .rev()
            .map(|(i, c)| {
                let aux = &self.value;
                match c {
                    '1' => {
                        Binary::new(String::from(aux.to_string() + &"0".repeat(length - i)), '0')
                    }
                    _ => Binary::new(String::from("0".to_string()), '0'),
                }
            })
            .reduce(|acc, b| acc + b)
            .unwrap_or_default();
        if self.signal != other.signal {
            r = r.change_signal()
        }
        return r;
    }
}
```

