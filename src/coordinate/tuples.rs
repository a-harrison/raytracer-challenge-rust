#![allow(unused)]

use std::ops::{ Add, Sub, Neg, Mul, Div };
use crate::coordinate::traits_old:: { Coordinate };
use crate::coordinate::constants::fuzzy_equal;

#[derive(Debug)]
pub struct ImageTuple { 
    pub coords: (f64, f64, f64, f64),
}

impl ImageTuple {
    pub fn point(x: f64, y: f64, z: f64) -> ImageTuple {
        ImageTuple { coords: (x, y, z, 1.0 ) }
    }

    pub fn vector(x: f64, y: f64, z: f64) -> ImageTuple {
        ImageTuple { coords: (x, y, z, 0.0 ) }
    } 

    pub fn tuple(x: f64, y: f64, z: f64, w: f64) -> ImageTuple {
        ImageTuple { coords: (x, y, z, w )}
    }
}

impl PartialEq for ImageTuple {
    fn eq(&self, other: &Self) -> bool {
        fuzzy_equal(self.x(), other.x()) &&
        fuzzy_equal(self.y(),  other.y()) &&
        fuzzy_equal(self.z(), other.z()) &&
        self.w() == other.w()
    }
}

// impl <T: Add<Output = T>> Add for ImageTuple<T> {
//     type Output = Self;

//     fn add(self, rhs: Self) -> Self::Output {
//         Self {
//             coords: (
//                 self.x() + rhs.x,
//                 self.y() + rhs.y,
//                 self.z() + rhs.z,
//                 self.w() + rhs.w                    
//             )
//         }
//     }
// }

impl Add for ImageTuple {
    type Output = Self; 

    fn add(self, rhs: Self) -> Self {        
        // TODO: Can we iterate over all fields in a tuple?
        Self  { 
            coords: (
                (self.x() + rhs.x()), 
                (self.y() + rhs.y()), 
                (self.z() + rhs.z()),
                (self.w() + rhs.w())
            )
        }
    }
}

impl Sub for ImageTuple {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            coords: (
                (self.x() - rhs.x()), 
                (self.y() - rhs.y()), 
                (self.z() - rhs.z()),
                (self.w() - rhs.w())
            )
        }
    }
}

impl Neg for ImageTuple {
    type Output = Self; 

    fn neg(self) -> Self {
        Self {
            coords: (
                -self.x(), -self.y(), -self.z(), -self.w()
            )
        }
    }
}

impl Mul<f64> for ImageTuple {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            coords: (
                (self.x() * rhs as f64), 
                (self.y() * rhs as f64), 
                (self.z() * rhs as f64),
                (self.w() * rhs as f64)
            )
        }
    }
}

impl Div<f64> for ImageTuple {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            coords: (
                (self.x() / rhs as f64), 
                (self.y() / rhs as f64), 
                (self.z() / rhs as f64),
                (self.w() / rhs as f64)
            )
        }
    }
}

impl Mul<ImageTuple> for f64 {
    type Output = ImageTuple;

    fn mul(self, rhs: ImageTuple) -> ImageTuple {
        rhs * self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_is_a_point() {
        let a: ImageTuple = ImageTuple::tuple(4.3, -4.2, 3.1, 1.0);
        assert_eq!(a.x(), 4.3);
        assert_eq!(a.y(), -4.2);
        assert_eq!(a.z(), 3.1);
        assert_eq!(a.is_point(), true);
        assert_eq!(a.is_vector(), false);
    }

    #[test]
    fn tuples_is_a_vector() {
        let a: ImageTuple = ImageTuple::tuple(4.3, -4.2, 3.1, 0.0);
        assert_eq!(a.x(), 4.3);
        assert_eq!(a.y(), -4.2);
        assert_eq!(a.z(), 3.1);
        assert_eq!(a.is_point(), false);
        assert_eq!(a.is_vector(), true);    
    }

    #[test]
    fn point_creates_tuple_with_w_1() {
        let p = ImageTuple::point(4.0, -4.0, 3.0);
        assert_eq!(p, ImageTuple::tuple(4.0, -4.0, 3.0, 1.0));
    }

    #[test]
    fn vector_creates_tuple_with_w_1() {
        let p = ImageTuple::vector(4.0, -4.0, 3.0);
        assert_eq!(p, ImageTuple::tuple(4.0, -4.0, 3.0, 0.0));
    }
    
    #[test]
    fn adding_two_tuples() {
        let a1 = ImageTuple::tuple(3.0, -2.0, 5.0, 1.0);
        let a2 = ImageTuple::tuple(-2.0, 3.0, 1.0, 0.0);

        assert_eq!(a1 + a2, ImageTuple::tuple(1.0, 1.0, 6.0, 1.0 )); 
    }

    #[test]
    fn subtracting_two_points() {
        let p1: ImageTuple = ImageTuple::point(3.0, 2.0, 1.0); 
        let p2: ImageTuple = ImageTuple::point(5.0, 6.0, 7.0); 

        assert_eq!(p1 - p2, ImageTuple::vector(-2.0, -4.0, -6.0)); 
    }

    #[test]
    fn subtracting_vector_from_point() {
        let p1: ImageTuple = ImageTuple::point(3.0, 2.0, 1.0); 
        let p2: ImageTuple = ImageTuple::vector(5.0, 6.0, 7.0); 

        assert_eq!(p1 - p2, ImageTuple::point(-2.0, -4.0, -6.0)); 
    }

    #[test]
    fn subtracting_two_vectors() {
        let p1: ImageTuple = ImageTuple::vector(3.0, 2.0, 1.0); 
        let p2: ImageTuple = ImageTuple::vector(5.0, 6.0, 7.0); 

        assert_eq!(p1 - p2, ImageTuple::vector(-2.0, -4.0, -6.0)); 
    }

    #[test]
    fn subtracting_vector_from_zero_vector() {
        let zero = ImageTuple::vector(0.0, 0.0, 0.0);
        let v = ImageTuple::vector(1.0, -2.0, 3.0);

        assert_eq!(zero - v, ImageTuple::vector(-1.0, 2.0, -3.0));
    }

    #[test]
    fn negating_a_tuple() {
        let a = ImageTuple::tuple(1.0, 2.0, 3.0, 4.0);
        assert_eq!(-a, ImageTuple::tuple(-1.0, -2.0, -3.0, -4.0));
    }

    #[test]
    fn multiplying_a_tuple_by_a_scalar() {
        let a: ImageTuple = ImageTuple::tuple(1., -2.0, 3.0, -4.0); 
        assert_eq!(a * 3.5, ImageTuple::tuple(3.5, -7.0, 10.5, -14.0));
    }

    #[test]
    fn multiplying_a_scalar_by_a_tuple() {
        let a: ImageTuple = ImageTuple::tuple(1., -2.0, 3.0, -4.0); 
        assert_eq!(3.5 * a, ImageTuple::tuple(3.5, -7.0, 10.5, -14.0));
    }

    #[test]
    fn multiplying_a_tuple_by_a_fraction() {
        let a: ImageTuple = ImageTuple::tuple(1., -2.0, 3.0, -4.0); 
        assert_eq!(a * 0.5, ImageTuple::tuple(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn dividing_a_tuple_by_a_scalar() {
        let a: ImageTuple = ImageTuple::tuple(1., -2.0, 3.0, -4.0); 
        assert_eq!(a / 2.0, ImageTuple::tuple(0.5, -1.0, 1.5, -2.0));
    }
}
