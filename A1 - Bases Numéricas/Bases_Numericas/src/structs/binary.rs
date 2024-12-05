use std::{
    cmp::Ordering,
    ops::{Add, BitAnd, BitOr, BitXor, Mul, Sub},
};

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

    pub fn next(&self) -> Binary {
        if self.value == '0'.to_string() {
            return Binary::new("1".to_string(), '0');
        }
        match self.signal {
            '0' => self.next_positive(),
            _ => self.next_negative(),
        }
    }

    fn next_positive(&self) -> Binary {
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

    fn next_negative(&self) -> Binary {
        let mut bits: Vec<char> = self.value.chars().collect();
        let mut borrow = true;
        for i in (0..bits.len()).rev() {
            if borrow {
                match bits[i] {
                    '0' => {
                        bits[i] = '1';
                    }
                    '1' => {
                        bits[i] = '0';
                        borrow = false
                    }
                    _ => panic!("invalid binary Binary"),
                }
            }
        }
        let result: String = bits.iter().collect();
        let mut signal: char = self.signal;
        let aux: String = result.chars().filter(|&c| c != '0').collect();
        if aux.is_empty() {
            signal = '0';
        }
        Binary::new(result, signal)
    }

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

    fn pad_compare(&self, other: &Self) -> (Binary, Binary) {
        let max_len = self.value.len().max(other.value.len());

        let mut x_padded = self.clone();
        let mut y_padded = other.clone();
        x_padded.value = format!("{:0>width$}", x_padded.value, width = max_len);
        y_padded.value = format!("{:0>width$}", y_padded.value, width = max_len);
        (x_padded, y_padded)
    }

    pub fn two_complement(&self) -> Binary {
        if self.value == "0" {
            return Binary::new("0".to_string(), '0');
        }
        let result = self.invert().next_positive().change_signal();
        result
    }

    fn change_signal(self) -> Binary {
        let new = match self.signal {
            '1' => '0',
            '0' => '1',
            _ => '1',
        };
        return Binary::new(self.value, new);
    }

    fn greater_value(&self, other: &Self) -> (Binary, Binary) {
        match self.clone().value > other.clone().value {
            true => (self.clone(), other.clone()),
            _ => (other.clone(), self.clone()),
        }
    }

    fn treat_zero(&self) -> Binary {
        if self.not_zero() {
            self.clone()
        } else {
            Binary::new("0".to_string(), '0')
        }
    }

    fn not_zero(&self) -> bool {
        !self.clone().value.chars().all(|bit| bit == '0')
    }

    fn trim(&self) -> Binary {
        if self.not_zero() {
            let mut trimmed = self.clone();
            trimmed.value = trimmed.value.trim_start_matches('0').to_string();
            trimmed
        } else {
            self.treat_zero()
        }
    }

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
// fn subtract(self, other: &Self) -> Binary {
//     let mut result = String::new();
//     let (x, y) = Binary::pad_compare(&self, other);
//     let mut carry = false;

//     for (xb, yb) in x.chars().rev().zip(y.chars().rev()).map(|(a, b)| {
//         (
//             Binary::new(a.to_string(), '0'),
//             Binary::new(b.to_string(), '0'),
//         )
//     }) {
//         let x = &xb.value;
//         let y = &yb.value;
//         let xor: &str =
//             &(Binary::new(x.to_string(), '0') ^ Binary::new(y.to_string(), '0')).value;
//         let and: &str =
//             &(Binary::new(x.to_string(), '0') & Binary::new(y.to_string(), '0')).value;
//         match xor {
//             "1" => {
//                 if carry {
//                     result.push('0');
//                 } else {
//                     result.push('1');
//                     carry = false;
//                 }
//             }
//             "0" => {
//                 if carry {
//                     result.push('1');
//                 } else {
//                     result.push('0');
//                 }
//                 if and == '1'.to_string() {
//                     carry = true;
//                 } else {
//                     carry = false;
//                 }
//             }
//             _ => panic!("invalid binary character"),
//         }
//     }
//     if carry != false {
//         result.push('1');
//     }
//     Binary::new(result.chars().rev().collect(), '0')
// }

// fn module(self) -> Binary {
//     Binary::new(self.value, '0')
// }
// }
impl Default for Binary {
    fn default() -> Self {
        Binary::new("0".to_string(), '0')
    }
}
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

impl Add for Binary {
    type Output = Binary;
    fn add(self, other: Self) -> Self::Output {
        match (self.signal, other.signal) {
            ('0', '0') => {
                let (x, y) = self.pad_compare(&other);
                x.addinng(&y)
            }
            ('0', '1') | ('1', '0') => {

                let (x, y) = self.greater_value(&other);
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
impl Sub for Binary {
    type Output = Binary;
    fn sub(self, other: Self) -> Self::Output {
        self + other.change_signal()
    }
}
impl Mul for Binary {
    type Output = Binary;

    fn mul(self, other: Self) -> Self::Output {
        let length = other.value.len() -1;
        let mut r = other
            .value
            .char_indices()
            .rev()
            .map(|(i, c)| {
                let aux = &self.value;
                println!("Caractere {}: {} | {}",i.to_string(), c, aux.to_string() + &"0".repeat(length - i));
                match c {
                    '1' => Binary::new(String::from( aux.to_string() + &"0".repeat(length - i)), '0'),
                    _ => Binary::new(String::from( "0".to_string() ), '0')
                }
                // let aux_str = self.value.clone();
                
            })
            .reduce(|acc, b| {
                println!("{} + {}",acc.get_binary(), b.get_binary());
                acc + b
            })
            .unwrap_or_default();
        if self.signal != other.signal{
            r = r.change_signal()
        }
        return r;
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
        self.get_binary().trim() == other.get_binary().trim()
    }
}
impl Eq for Binary {}

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
        match Self::validate(&value) {
            Ok(()) => Some(Binary::new(value, signal.chars().next().unwrap())),
            Err(_e) => {
                return None;
            }
        }
    }
    fn validate(value: &str) -> Result<(), &str> {
        if value.chars().all(|c| c == '0' || c == '1') {
            Ok(())
        } else {
            Err("Invalid binary")
        }
    }
}

// M + m = M + m
// -M + (-m) = -(M + m)
// M.s == m.s = (M + m).s

// M + (- m) = M - m
// -M + m = -(M - m)
// M.s != m.s = (M - m).s(M)

// 101 - 1

// 101
// 111
// 100

// 111 - 11 = 100  (7 - 3 = 4)

// 0011 -> 1100 -> 1101
// 0111 + 1101 = (1)0100
// 111 + 101 = (1)100

// 111 +  (-11) = 100  (7 + (-3)  = 4)

// 0011 -> 1100 -> 1101
// 0111 + 1101 = (1)0100
// 111 + 101 = (1)100

// -1001 + (-0111) = -10000  ( -9 + (-7)  = -16)

// 1. 11001 -> 00110 -> 00111
// 2. 10111 -> 01000 -> 01001
// 00111 + 01001 = 010000 -> 110000
// 111 + 101 = (1)100

// 11 - 10 = 1  (3 - 2 = 1)

// 110 -> 001 -> 010
// 11 + 10 = (1)01 -> 001

// 1 - 1 = 1  (1 + (-1) = 1)

// 110 -> 001 -> 010
// 11 + 10 = (1)01 -> 001

// 1111 - 11 = 1111 + (-11)

// +0 | 0000
// +1 | 0001
// +2 | 0010
// +3 | 0011
// +4 | 0100
// +5 | 0101
// +6 | 0110
// +7 | 0111
// -8 | 1000
// -7 | 1001
// -6 | 1010
// -5 | 1011
// -4 | 1100
// -3 | 1101
// -2 | 1110
// -1 | 1111

//          10
//          11
//          __
//          10
//         10
//         101
