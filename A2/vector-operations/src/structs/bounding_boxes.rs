use super::vector::Vector;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub enum BoundingType {
    AABB,
    OBB,
    Sphere,
    Capsule,
    Unknown,
}

impl BoundingType {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "aabb" => BoundingType::AABB,
            "obb" => BoundingType::OBB,
            "sphere" => BoundingType::Sphere,
            "capsule" => BoundingType::Capsule,
            _ => BoundingType::Unknown,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AABB {
    min: Vector,
    max: Vector,
}

impl AABB {
    pub fn from_points(points: &[Vector]) -> Option<Self> {
        if points.is_empty() {
            return None;
        }
        if let Some((mut min_x, mut min_y)) = Self::get_point_xny(&points[0]) {
            let mut max_x = min_x;
            let mut max_y = min_y;

            for p in points.iter().skip(1) {
                if let Some((px, py)) = Self::get_point_xny(p) {
                    if px < min_x {
                        min_x = px;
                    }
                    if px > max_x {
                        max_x = px;
                    }
                    if py < min_y {
                        min_y = py;
                    }
                    if py > max_y {
                        max_y = py;
                    }
                }
            }

            Some(AABB {
                min: Vector::new(vec![min_x, min_y]),
                max: Vector::new(vec![max_x, max_y]),
            })
        } else {
            None
        }
    }

    fn get_point_xny(point: &Vector) -> Option<(f64, f64)> {
        if let (Some(x), Some(y)) = (point.get(0), point.get(1)) {
            Some((x, y))
        } else {
            None
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Sphere {
    center: Vector,
    radius: f64,
}

impl Sphere {
    pub fn from_points(points: &[Vector]) -> Option<Self> {
        if points.is_empty() {
            return None;
        }

        let mut sum_x: f64 = 0.0;
        let mut sum_y: f64 = 0.0;
        let count = points.len() as f64;
        points.iter().for_each(|p| {
            if let (Some(x), Some(y)) = (p.get(0), p.get(1)) {
                sum_x += x;
                sum_y += y;
            }
        });

        let center = Vector::new(vec![sum_x / count, sum_y / count]);

        let radius2 = points
            .iter()
            .filter_map(|p| {
                let (px, py) = (p.get(0)?, p.get(1)?);
                let (cx, cy) = (center.get(0)?, center.get(1)?);
                Some((px - cx).powi(2) + (py - cy).powi(2))
            })
            .fold(0.0, f64::max);
        let radius = radius2.sqrt();
        Some(Sphere { center, radius })
    }
}
