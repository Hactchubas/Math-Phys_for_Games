use std::ops::{Add, Mul, Sub};

use serde::{Deserialize, Serialize};

use super::elements::LineSegment;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector {
    dimensions: Vec<f64>,
}

impl Vector {
    fn new(dims: Vec<f64>) -> Self {
        Vector { dimensions: dims }
    }

    // pub fn subtract(&self, other: &Self) -> Self {
    //     let (new_self, new_other) = self.equalize_dimensions(&other);

    //     Vector::new(
    //         new_self
    //             .dimensions
    //             .iter()
    //             .zip(new_other.dimensions.iter())
    //             .map(|(s_di, o_di)| s_di - o_di)
    //             .collect(),
    //     )
    // }

    pub fn cardinality(&self) -> usize {
        self.dimensions.len()
    }

    pub fn signal_in_r(&self, r: usize) -> bool {
        if r > self.cardinality() {
            false
        } else {
            matches!(self.dimensions.get(r-1), Some(&value) if value > 0.0)
        }
    }

    fn modulus(&self) -> f64 {
        self.dimensions
            .iter()
            .map(|&di| di.powi(2))
            .sum::<f64>()
            .sqrt()
    }

    fn unit(&self) -> Self {
        self * (1 as f64 / self.modulus())
    }

    fn equalize_dimensions(&self, other: &Self) -> (Self, Self) {
        let mut new_self = self.clone();
        let mut new_other = other.clone();
        let self_len = new_self.dimensions.len();
        let other_len = new_other.dimensions.len();

        if self_len < other_len {
            new_self
                .dimensions
                .extend(vec![0 as f64; other_len - self_len]);
            (new_self, new_other)
        } else if self_len > other_len {
            new_other
                .dimensions
                .extend(vec![0 as f64; self_len - other_len]);
            (new_self, new_other)
        } else {
            (new_self, new_other)
        }
    }

    // fn not_zero(&self) -> bool {
    //     !self.dimensions.iter().all(|&di| di == 0 as f64)
    // }

    pub fn cross_product(&self, other: &Self) -> Option<Self> {
        match (self.dimensions.len(), other.dimensions.len()) {
            (2, 2) => {
                let result: Vec<f64> = vec![
                    0.0,
                    0.0,
                    self.dimensions[0] * other.dimensions[1]
                        - self.dimensions[1] * other.dimensions[0],
                ];
                Some(Vector::new(result))
            }
            (3, 3) => {
                let result = vec![
                    self.dimensions[1] * other.dimensions[2]
                        - self.dimensions[2] * other.dimensions[1],
                    self.dimensions[2] * other.dimensions[0]
                        - self.dimensions[0] * other.dimensions[2],
                    self.dimensions[0] * other.dimensions[1]
                        - self.dimensions[1] * other.dimensions[0],
                ];
                Some(Vector::new(result))
            }
            _ => None,
        }
    }

    pub fn dot_product(&self, other: &Self) -> Option<f64> {
        let (new_self, new_other) = self.equalize_dimensions(other);

        let result = new_self
            .dimensions
            .iter()
            .zip(new_other.dimensions.iter())
            .map(|(s_di, o_di)| s_di * o_di)
            .sum();

        Some(result)
    }

    pub fn projected_at(&self, other: &Self) -> Option<Self> {
        let unitary_other = other.unit();
        if let Some(p) = self.dot_product(&unitary_other) {
            let res = &unitary_other * p;
            Some(res)
        } else {
            None
        }
    }

    // pub fn reflected_at(&self, other: &Self) -> Option<Self> {
    //     if let Some(projected) = self.projected_at(other) {
    //         Some(self.clone() - projected.scale(2.0))
    //     } else {
    //         None
    //     }
    // }

    pub fn decompose(&self, other: &Self) -> Option<(Self, Self)> {
        if let Some(projected) = self.projected_at(other) {
            let orthogonal = self - &&projected;
            Some((orthogonal, projected))
        } else {
            None
        }
    }

    pub fn parameterized_reaction(&self, alpha: f64, other: &Self, beta: f64) -> Option<Self> {
        if let Some((vn, vp)) = self.decompose(other) {
            let n = &vn * alpha;
            let p = &vp * (-beta);
            let reac = n + p;
            Some(reac)
        } else {
            None
        }
    }

    pub fn to_line_segment(&self, other: &Self) -> LineSegment {
        LineSegment::new(self.clone(), other.clone())
    }

    pub fn normal_vec(&self) -> Option<Self> {
        let mut dims = vec![0.0; self.cardinality() - 1];
        dims.insert(0, 1.0);
        let other = &Vector::new(dims);
        if let Some((vn, vp)) = self.decompose(other) {
            let n = vp - vn;
            Some(n.unit())
        } else {
            None
        }
    }
}

impl Add<&Self> for &Vector {
    type Output = Vector;
    fn add(self, other: &Self) -> Self::Output {
        let (new_self, new_other) = self.equalize_dimensions(other);
        Vector::new(
            new_self
                .dimensions
                .iter()
                .zip(new_other.dimensions.iter())
                .map(|(s_di, o_di)| s_di + o_di)
                .collect(),
        )
    }
}
impl Add<&Self> for Vector {
    type Output = Self;
    fn add(self, other: &Self) -> Self::Output {
        &self + &other
    }
}
impl Add<Self> for Vector {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        self + &other
    }
}

impl Sub<Self> for &Vector {
    type Output = Vector;
    fn sub(self, other: Self) -> Self::Output {
        let (new_self, new_other) = self.equalize_dimensions(&other);
        Vector::new(
            new_self
                .dimensions
                .iter()
                .zip(new_other.dimensions.iter())
                .map(|(s_di, o_di)| s_di - o_di)
                .collect(),
        )
    }
}
impl Sub<Self> for Vector {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        &self - &other
    }
}
// impl Sub<Self> for &Vector {
//     type Output = Self;
//     fn sub(self, other: Self) -> Self::Output {
//         self - &other
//     }
// }

impl Mul<&Self> for &Vector {
    type Output = f64;
    fn mul(self, other: &Self) -> Self::Output {
        let (new_self, new_other) = self.equalize_dimensions(other);
        new_self
            .dimensions
            .iter()
            .zip(new_other.dimensions.iter())
            .map(|(s_di, o_di)| s_di - o_di)
            .sum::<f64>()
    }
}
impl Mul<f64> for &Vector {
    type Output = Vector;
    fn mul(self, m: f64) -> Self::Output {
        let result = self.dimensions.iter().map(|&di| di * m).collect();
        Vector::new(result)
    }
}

impl Eq for &Vector {}

impl PartialEq for &Vector {
    fn eq(&self, other: &Self) -> bool {
        let (new_self, new_other) = self.equalize_dimensions(other);
        new_self
            .dimensions
            .iter()
            .zip(new_other.dimensions.iter())
            .map(|(s_di, o_di)| s_di == o_di)
            .all(|v| v)
    }
}
