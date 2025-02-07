use std::{
    f64::consts::PI,
    ops::{Add, Mul, Sub},
};

use serde::{Deserialize, Serialize};

use super::elements::{self, LineSegment};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector {
    dimensions: Vec<f64>,
}

impl Vector {
    fn new(dims: Vec<f64>) -> Self {
        Vector { dimensions: dims }
    }

    /// Return the number of dimensions of Self
    pub fn cardinality(&self) -> usize {
        self.dimensions.len()
    }

    /// Return if teh value of dimension { r } is positive
    pub fn signal_in_r(&self, r: usize) -> bool {
        if r > self.cardinality() {
            false
        } else {
            matches!(self.dimensions.get(r-1), Some(&value) if value > 0.0)
        }
    }

    /// Return the modulus of Self
    fn modulus(&self) -> f64 {
        self.dimensions
            .iter()
            .map(|&di| di.powi(2))
            .sum::<f64>()
            .sqrt()
    }

    /// Return the unitary vector of Self
    fn unit(&self) -> Self {
        self * (1 as f64 / self.modulus())
    }

    /// Return tuple of two new Vectors equal to the given {self} and {other} with equal cardinalities
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
        Some(Vector::new(vec![-self.dimensions[1], self.dimensions[0]]))
    }

    fn angle_dot(&self, other: &Self) -> Option<f64> {
        let mag_a = self.modulus();
        let mag_b = other.modulus();
        if mag_a == 0.0 || mag_b == 0.0 {
            return None;
        }
        if let Some(mut cos_theta) = self.dot_product(other) {
            cos_theta = cos_theta / (mag_a * mag_b);
            cos_theta = cos_theta.clamp(-1.0, 1.0);
            Some(cos_theta.acos() * 180.0 / PI)
        } else {
            None
        }
    }

    fn angle_cross(&self, other: &Self) -> Option<f64> {
        let mag_a = self.modulus();
        let mag_b = other.modulus();
        if mag_a == 0.0 || mag_b == 0.0 {
            return None;
        }
        if let Some(vec) = self.cross_product(other) {
            let mut sin_theta = vec.dimensions[2] / (mag_a * mag_b);
            sin_theta = sin_theta.clamp(-1.0, 1.0);
            Some(sin_theta.asin() * 180.0 / PI)
        } else {
            None
        }
    }

    fn angle_both(&self, other: &Self) -> Option<f64> {
        let angle_dot = self.angle_dot(other);
        let angle_cross = self.angle_cross(other);
        match (angle_dot, angle_cross) {
            (Some(dot_angle), Some(cross_angle)) =>{
                let angle = dot_angle * cross_angle.signum(); // Retorna o ângulo correto [-π, π]
                Some(angle) 
            }
            ( _ , _ ) => None
        }
    }

    pub fn angle_between(&self, other: &Self, method: usize) -> Result<f64, &str> {
        match method {
            1 => {
                if let Some(angle) = self.angle_dot(other) {
                    Ok(angle)
                } else {
                    Err("Could not calculate angle by dot method")
                }
            }
            2 => {
                if let Some(angle) = self.angle_cross(other) {
                    Ok(angle)
                } else {
                    Err("Could not calculate angle by cross method")
                }
            }
            3 => {
                if let Some(angle) = self.angle_both(other) {
                    Ok(angle)
                } else {
                    Err("Could not calculate angle by both methods together")
                }
            }
            _ => Err("Method not valid, use => 1: odt, 2: cross, 3: both"),
        }

    }
    
    fn pseudo_angle(&self) -> f64 {
        if self.dimensions[0] == 0.0 && self.dimensions[1] == 0.0 {
            return 0.0;
        }

        let norm_x = self.dimensions[0] / self.modulus();
        let norm_y = self.dimensions[1]  / self.modulus();

        if norm_y >= 0.0 {
            if norm_x >= 0.0 {
                norm_y / (norm_x + norm_y) // Quadrante 1
            } else {
                1.0 - norm_x / (-norm_x + norm_y) // Quadrante 2
            }
        } else {
            if norm_x < 0.0 {
                2.0 + (-norm_y) / (-norm_x - norm_y) // Quadrante 3
            } else {
                3.0 + norm_x / (norm_x - norm_y) // Quadrante 4
            }
        }
    }
    /// **Calcula o pseudoângulo entre dois vetores no quadrado [0,8).**
    fn pseudo_angle_between(&self, other: Vector) -> f64 {
        let pa1 = self.pseudo_angle();
        let pa2 = other.pseudo_angle();
        let diff = pa2 - pa1;
        if diff < 0.0 { diff + 8.0 } else { diff }
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

impl PartialEq for Vector {
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

impl Eq for Vector {}
impl std::hash::Hash for Vector {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for dim in &self.dimensions {
            state.write_u64(dim.to_bits());
        }
    }
}
