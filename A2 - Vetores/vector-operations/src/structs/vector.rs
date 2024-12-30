use std::ops::{Add, Mul, Sub};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector {
    dimensions: Vec<f64>,
}

impl Vector {
    fn new(dims: Vec<f64>) -> Self {
        Vector { dimensions: dims }
    }

    fn modulus(&self) -> f64 {
        self.dimensions
            .iter()
            .map(|&di| di.powi(2))
            .sum::<f64>()
            .sqrt()
    }

    fn invert(&self) -> Self {
        Vector::new(self.dimensions.iter().map(|&di| -di).collect())
    }

    pub fn scale(&self, m: f64) -> Vector {
        let result = self.dimensions.iter().map(|&di| di * m).collect();

        Vector::new(result)
    }

    fn unit(&self) -> Self {
        self.scale(1 as f64 / self.modulus()).clone()
    }

    fn equalize_dimensions(&self, other: &Self) -> (Self, Self) {
        let this_length = self.dimensions.len();
        let other_length = other.dimensions.len();

        let mut smaller: Vector;
        if this_length < other_length {
            smaller = self.clone();
            smaller
                .dimensions
                .extend(vec![0 as f64; other_length - this_length]);

            (smaller.clone(), other.clone())
        } else if this_length > other_length {
            smaller = other.clone();
            smaller
                .dimensions
                .extend(vec![0 as f64; this_length - other_length]);

            (self.clone(), smaller)
        } else {
            (self.clone(), other.clone())
        }
    }

    fn not_zero(&self) -> bool {
        !self.dimensions.iter().all(|&di| di == 0 as f64)
    }

    pub fn cross_product(&self, other: &Vector) -> Option<Vector> {
        if self.dimensions.len() != 3 || other.dimensions.len() != 3 {
            return None;
        }

        let result = vec![
            self.dimensions[1] * other.dimensions[2] - self.dimensions[2] * other.dimensions[1],
            self.dimensions[2] * other.dimensions[0] - self.dimensions[0] * other.dimensions[2],
            self.dimensions[0] * other.dimensions[1] - self.dimensions[1] * other.dimensions[0]
        ];

        Some(Vector::new(result))
    }

    pub fn dot_product(&self, other: &Vector) -> Option<f64> {
        let ( new_self , new_other ) = self.equalize_dimensions(other);

        let result = new_self.dimensions.iter()
            .zip(new_other.dimensions.iter())
            .map(|(s_di, o_di)| s_di * o_di)
            .sum();

        Some(result)
    }

    pub fn projected_at(&self, other: &Self) -> Option<Self> {
        let unitary_other = other.unit();
        if let Some( p ) = 
            self.dot_product(&unitary_other) {
            let res = unitary_other.scale(p);
            Some(res)
        } else {
            None
        }
    }

    pub fn reflected_at(&self, other: &Self) -> Option<Self> {
        if let Some(projected) = self.projected_at(other) {
            Some( self.clone() - projected.scale(2.0) )
        } else {
            None
        }
    }

}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Self) -> Self::Output {
        let (new_self, new_other) = self.equalize_dimensions(&other);

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

impl Sub for Vector {
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

impl Mul for Vector {
    type Output = f64;

    fn mul(self, other: Self) -> Self::Output {
        let (new_self, new_other) = self.equalize_dimensions(&other);

        new_self
            .dimensions
            .iter()
            .zip(new_other.dimensions.iter())
            .map(|(s_di, o_di)| s_di - o_di)
            .sum::<f64>()
    }
}
