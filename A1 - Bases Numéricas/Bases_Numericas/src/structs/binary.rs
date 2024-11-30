use std::ops::{Add, BitAnd, BitOr, BitXor};

#[derive(Debug, Clone, PartialEq)]
pub struct Binary {
    pub value: String,
}

impl Binary {
    pub fn new(value: &str) -> Self {
        Binary {
            value: value.to_string(),
        }
    }

    pub fn is_valid_binary(value: &str) -> bool {
        value.chars().all(|bit| bit == '0' || bit == '1')
    }

    pub fn next(self) -> Binary {
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
                    _ => panic!("invalid binary character"),
                }
            }
        }
        if carry {
            bits.insert(0, '1');
        }
        let result: String = bits.iter().collect();
        Binary::new(&result)
    }

    pub fn invert(self) -> Binary {
        let mut bits: Vec<char> = self.value.chars().collect();
        for i in 0..bits.len() {
            match bits[i] {
                '0' => bits[i] = '1',
                '1' => bits[i] = '0',
                _ => panic!("invalid binary character"),
            }
        }
        let result: String = bits.iter().collect();
        Binary::new(&result)
    }

    fn pad_compare(self, other: Self) -> (String, String) {
        let max_len = self.value.len().max(other.value.len());
        let x_padded = format!("{:0>width$}", self.value, width = max_len); // Preenche com '0' à esquerda
        let y_padded = format!("{:0>width$}", other.value, width = max_len); // Preenche com '0' à esquerda
        println!("x: {}, y: {}, max: {}", x_padded, y_padded, max_len);
        (x_padded, y_padded)
    }
}

impl BitAnd for Binary {
    type Output = Binary;

    fn bitand(self, other: Self) -> Self::Output {
        let (x, y) = Binary::pad_compare(self, other);

        let result: String = x
            .chars()
            .zip(y.chars())
            .map(|(a, b)| if a == '1' && b == '1' { '1' } else { '0' })
            .collect();

        Binary::new(&result)
    }
}

impl BitOr for Binary {
    type Output = Binary;
    fn bitor(self, other: Self) -> Self::Output {
        let (x, y) = Binary::pad_compare(self, other);

        let result: String = x
            .chars()
            .zip(y.chars())
            .map(|(a, b)| if a == '1' || b == '1' { '1' } else { '0' })
            .collect();

        Binary::new(&result)
    }
}

impl BitXor for Binary {
    type Output = Binary;
    fn bitxor(self, other: Self) -> Self::Output {
        let (x, y) = Binary::pad_compare(self, other);

        let result: String = x
            .chars()
            .zip(y.chars())
            .map(|(a, b)| if a != b { '1' } else { '0' })
            .collect();

        Binary::new(&result)
    }
}

impl Add for Binary {
    type Output = Binary;
    fn add(self, other: Self) -> Self::Output {
        let mut result = String::new();
        let (x, y) = Binary::pad_compare(self, other);
        let mut carry = false;

        for (xb, yb) in x
            .chars()
            .rev()
            .zip(y.chars().rev())
            .map(|(a, b)| (Binary::new(&a.to_string()), Binary::new(&b.to_string())))
        {
            let xor: &str = &(Binary::new(&xb.value) ^ Binary::new(&yb.value)).value;
            let and: &str = &(Binary::new(&xb.value) & Binary::new(&yb.value)).value;

            match xor {
                "1" => {
                    if carry {
                        result.push('0');
                    } else {
                        result.push('1');
                        carry = false;
                    }
                }
                "0" => {
                    if carry {
                        result.push('1');
                    } else {
                        result.push('0');
                        carry = false;
                    }
                    if and == '1'.to_string() {
                        carry = true;
                    } else {
                        result.push('1');
                        carry = false;
                    }
                }
                _ => panic!("invalid binary character"),
            }
        }
        if carry != false {
            result.push('1');
        }

        Binary::new(&result)
    }
}
