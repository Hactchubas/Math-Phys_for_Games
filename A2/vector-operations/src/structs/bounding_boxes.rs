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
        if points.len() == 1 {
            return Some(Sphere {
                center: points[0].clone(),
                radius: 0.0,
            });
        }
        
        // No need to shuffle since points are already random
        Some(welzl_helper(points, Vec::new()))
    }
}

// Helper function to construct circle from 0, 1, 2, or 3 points
fn make_circle(boundary: &[Vector]) -> Sphere {
    match boundary.len() {
        0 => Sphere {
            center: Vector::new(vec![0.0, 0.0]),
            radius: 0.0,
        },
        1 => Sphere {
            center: boundary[0].clone(),
            radius: 0.0,
        },
        2 => {
            let p1 = &boundary[0];
            let p2 = &boundary[1];
            let center_x = (p1.get(0).unwrap() + p2.get(0).unwrap()) / 2.0;
            let center_y = (p1.get(1).unwrap() + p2.get(1).unwrap()) / 2.0;
            let center = Vector::new(vec![center_x, center_y]);
            let radius = ((p1.get(0).unwrap() - center_x).powi(2) + 
                         (p1.get(1).unwrap() - center_y).powi(2)).sqrt();
            Sphere { center, radius }
        },
        3 => {
            // Calculate circumcircle of triangle
            let (p1, p2, p3) = (&boundary[0], &boundary[1], &boundary[2]);
            let (x1, y1) = (p1.get(0).unwrap(), p1.get(1).unwrap());
            let (x2, y2) = (p2.get(0).unwrap(), p2.get(1).unwrap());
            let (x3, y3) = (p3.get(0).unwrap(), p3.get(1).unwrap());

            let d = 2.0 * (x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2));
            if d.abs() < 1e-10 {  // Points are collinear
                return make_circle(&boundary[..2]);
            }

            let ux = ((x1.powi(2) + y1.powi(2)) * (y2 - y3) +
                     (x2.powi(2) + y2.powi(2)) * (y3 - y1) +
                     (x3.powi(2) + y3.powi(2)) * (y1 - y2)) / d;
            let uy = ((x1.powi(2) + y1.powi(2)) * (x3 - x2) +
                     (x2.powi(2) + y2.powi(2)) * (x1 - x3) +
                     (x3.powi(2) + y3.powi(2)) * (x2 - x1)) / d;

            let center = Vector::new(vec![ux, uy]);
            let radius = ((x1 - ux).powi(2) + (y1 - uy).powi(2)).sqrt();
            Sphere { center, radius }
        },
        _ => unreachable!(),
    }
}

// Main recursive Welzl's algorithm implementation
fn welzl_helper(points: &[Vector], mut boundary: Vec<Vector>) -> Sphere {
    // Base cases
    if points.is_empty() || boundary.len() == 3 {
        return make_circle(&boundary);
    }

    // Take the next point
    let p = points[0].clone();
    let remaining = &points[1..];

    // Recursively get the smallest circle without this point
    let circle = welzl_helper(remaining, boundary.clone());

    // If p is inside the circle, this is still the smallest circle
    if point_in_circle(&p, &circle) {
        return circle;
    }

    // Otherwise, p must be on the boundary of the minimal circle
    boundary.push(p);
    welzl_helper(remaining, boundary)
}

// Check if a point is inside a circle
fn point_in_circle(point: &Vector, circle: &Sphere) -> bool {
    let dx = point.get(0).unwrap() - circle.center.get(0).unwrap();
    let dy = point.get(1).unwrap() - circle.center.get(1).unwrap();
    (dx.powi(2) + dy.powi(2)).sqrt() <= circle.radius + 1e-10
}