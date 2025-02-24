use super::vector::Vector;
use serde::{Deserialize, Serialize};
use std::f64::{MAX, MIN};

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
        Some(Self::welzl_helper(points, Vec::new()))
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
                let radius = ((p1.get(0).unwrap() - center_x).powi(2)
                    + (p1.get(1).unwrap() - center_y).powi(2))
                .sqrt();
                Sphere { center, radius }
            }
            3 => {
                // Calculate circumcircle of triangle
                let (p1, p2, p3) = (&boundary[0], &boundary[1], &boundary[2]);
                let (x1, y1) = (p1.get(0).unwrap(), p1.get(1).unwrap());
                let (x2, y2) = (p2.get(0).unwrap(), p2.get(1).unwrap());
                let (x3, y3) = (p3.get(0).unwrap(), p3.get(1).unwrap());

                let d = 2.0 * (x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2));
                if d.abs() < 1e-10 {
                    // Points are collinear
                    return Self::make_circle(&boundary[..2]);
                }

                let ux = ((x1.powi(2) + y1.powi(2)) * (y2 - y3)
                    + (x2.powi(2) + y2.powi(2)) * (y3 - y1)
                    + (x3.powi(2) + y3.powi(2)) * (y1 - y2))
                    / d;
                let uy = ((x1.powi(2) + y1.powi(2)) * (x3 - x2)
                    + (x2.powi(2) + y2.powi(2)) * (x1 - x3)
                    + (x3.powi(2) + y3.powi(2)) * (x2 - x1))
                    / d;

                let center = Vector::new(vec![ux, uy]);
                let radius = ((x1 - ux).powi(2) + (y1 - uy).powi(2)).sqrt();
                Sphere { center, radius }
            }
            _ => unreachable!(),
        }
    }

    // Main recursive Welzl's algorithm implementation
    fn welzl_helper(points: &[Vector], mut boundary: Vec<Vector>) -> Sphere {
        // Base cases
        if points.is_empty() || boundary.len() == 3 {
            return Self::make_circle(&boundary);
        }

        // Take the next point
        let p = points[0].clone();
        let remaining = &points[1..];

        // Recursively get the smallest circle without this point
        let circle = Self::welzl_helper(remaining, boundary.clone());

        // If p is inside the circle, this is still the smallest circle
        if Self::point_in_circle(&p, &circle) {
            return circle;
        }

        // Otherwise, p must be on the boundary of the minimal circle
        boundary.push(p);
        Self::welzl_helper(remaining, boundary)
    }

    // Check if a point is inside a circle
    fn point_in_circle(point: &Vector, circle: &Sphere) -> bool {
        let dx = point.get(0).unwrap() - circle.center.get(0).unwrap();
        let dy = point.get(1).unwrap() - circle.center.get(1).unwrap();
        (dx.powi(2) + dy.powi(2)) <= circle.radius.powi(2)
    }
}

#[derive(Debug, Serialize)]
pub struct OBB {
    center: Vector,
    axes: [Vector; 2],
    half_sizes: Vector,
    points: [Vector; 4],
}

impl OBB {
    fn new(center: Vector, axes: [Vector; 2], half_sizes: Vector) -> Self {
        let center_aux = center.clone();
        let mut a = &center_aux + &&(&axes[0] * half_sizes.get(0).unwrap());
        a = a + &axes[1] * half_sizes.get(1).unwrap();
        let mut b = &center_aux - &&(&axes[0] * half_sizes.get(0).unwrap());
        b = b + &axes[1] * half_sizes.get(1).unwrap();
        let mut c = &center_aux - &&(&axes[0] * half_sizes.get(0).unwrap());
        c = c - &axes[1] * half_sizes.get(1).unwrap();
        let mut d = &center_aux + &&&(&axes[0] * half_sizes.get(0).unwrap());
        d = d - &axes[1] * half_sizes.get(1).unwrap();

        OBB {
            center: center,
            axes: [axes[0].clone(), axes[1].clone()],
            half_sizes,
            points: [a, b, c, d],
        }
    }

    /// Compute the Oriented Bounding Box (OBB) from a set of 2D points without third-party libraries.
    pub fn from_points(points: &[Vector]) -> Option<OBB> {
        let n: usize = points.len();

        // Step 1: Compute Centroid
        let mut centroid = points
            .iter()
            .fold(Vector::new(vec![0.0, 0.0]), |sum, p| sum + p);

        centroid = &centroid * (1.0 / n as f64);

        // Step 2: Compute Covariance Matrix
        let mut cov_xx = 0.0;
        let mut cov_xy = 0.0;
        let mut cov_yy = 0.0;

        for p in points {
            let diff = p - &centroid;
            if let Some((diff_x, diff_y)) = OBB::get_point_xny(&diff) {
                cov_xx += diff_x * diff_x;
                cov_xy += diff_x * diff_y;
                cov_yy += diff_y * diff_y;
            } else {
                return None;
            }
        }

        cov_xx /= n as f64;
        cov_xy /= n as f64;
        cov_yy /= n as f64;

        // Step 3: Compute Eigenvectors using an analytical method
        let trace = cov_xx + cov_yy;
        let det = cov_xx * cov_yy - cov_xy * cov_xy;
        let lambda1 = (trace / 2.0) + ((trace * trace / 4.0 - det).sqrt());

        let eigen_x = Vector::new(vec![cov_xy, lambda1 - cov_xx]).unit();

        let eigen_y = eigen_x.normal_vec().unwrap();

        // Step 4: Transform points into the new basis
        let mut final_min_x = MAX;
        let mut final_min_y = MAX;
        let mut final_max_x = MIN;
        let mut final_max_y = MIN;

        for p in points {
            let local_x = eigen_x.dot_product(&(p - &centroid)).unwrap();
            let local_y = eigen_y.dot_product(&(p - &centroid)).unwrap();

            final_min_x = final_min_x.min(local_x);
            final_min_y = final_min_y.min(local_y);
            final_max_x = final_max_x.max(local_x);
            final_max_y = final_max_y.max(local_y);
        }

        // Step 5: Compute OBB properties
        let mut half_sizes = Vector::new(vec![final_max_x, final_max_y])
            - Vector::new(vec![final_min_x, final_min_y]);
        half_sizes = &half_sizes * 0.5;

        let mut obb_center = centroid + (&(&eigen_x * (final_max_x + final_min_x)) * 0.5);
        obb_center = obb_center + (&(&eigen_y * (final_max_y + final_min_y)) * 0.5);


        Some(OBB::new(obb_center, [eigen_x, eigen_y], half_sizes))
    }

    fn get_point_xny(point: &Vector) -> Option<(f64, f64)> {
        if let (Some(x), Some(y)) = (point.get(0), point.get(1)) {
            Some((x, y))
        } else {
            None
        }
    }
}
