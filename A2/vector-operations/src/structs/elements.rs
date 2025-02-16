use super::vector::Vector;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineSegment {
    x: Vector,
    y: Vector,
}

impl LineSegment {
    pub fn new(x: Vector, y: Vector) -> Self {
        LineSegment { x, y }
    }

    pub fn get_a(&self) -> Vector {
        self.x.clone()
    }

    pub fn get_b(&self) -> Vector {
        self.y.clone()
    }

    pub fn get_normal(&self) -> Option<Vector> {
        let seg_vec = &self.x - &self.y;
        seg_vec.normal_vec()
    }

    pub fn vec_from_seg(&self) -> Vector {
        &self.y - &self.x
    }

    fn intersection_vec(&self, other: &Self) -> Option<Vector> {
        let c_vec = other.x.to_owned();
        let d_vec = other.y.to_owned();
        let b_vec = self.y.to_owned();
        let a_vec = self.x.to_owned();

        let cd = &c_vec - &d_vec;

        if let Some(normal) = cd.normal_vec() {
            match (
                (&c_vec - &a_vec).dot_product(&normal),
                (&b_vec - &a_vec).dot_product(&normal),
            ) {
                (Some(n), Some(d)) => {
                    let t = n / d;

                    Some((&(&b_vec - &a_vec) * t) + &a_vec)
                }
                (_, _) => None,
            }
        } else {
            None
        }
    }

    pub fn intersects(&self, other: &Self) -> Option<Vector> {
        let c_vec = other.x.to_owned();
        let d_vec = other.y.to_owned();
        let b_vec = self.y.to_owned();
        let a_vec = self.x.to_owned();

        let ac = &c_vec - &a_vec;
        let ad = &d_vec - &a_vec;
        let ab = &b_vec - &a_vec;

        let cd = &d_vec - &c_vec;
        let ca = &a_vec - &c_vec;
        let cb = &b_vec - &c_vec;

        match (ab.cross_product(&ac), ab.cross_product(&ad)) {
            (Some(ab_x_ac), Some(ab_x_ad)) if (ab_x_ac.signal_in_r(3) ^ ab_x_ad.signal_in_r(3)) => {
                match (cd.cross_product(&ca), cd.cross_product(&cb)) {
                    (Some(cd_x_ca), Some(cd_x_cb))
                        if (cd_x_ca.signal_in_r(3) ^ cd_x_cb.signal_in_r(3)) =>
                    {
                        self.intersection_vec(other)
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

impl PartialEq for LineSegment {
    fn eq(&self, other: &Self) -> bool {
        &self.x == &other.x && &self.y == &other.y
    }
}
