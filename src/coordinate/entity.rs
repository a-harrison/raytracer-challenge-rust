use std::ops::{ Add, Sub, Neg, Mul, Div };
use crate::coordinate::constants::fuzzy_equal;
use crate::coordinate::{ point::Point, traits::Coordinate, vector::Vector };

#[derive(Debug)]
pub struct Entity { values: (f64, f64, f64, f64) }

impl Entity {
    pub fn create(x: f64, y: f64, z: f64, w: f64) -> Entity {
        Self {
            values: (x, y, z, w)
        }
    }

    pub fn magnitude(&self) -> f64 {
        (
            (self.x() * self.x()) + 
            (self.y() * self.y()) +
            (self.z() * self.z()) + 
            (self.w() * self.w())
        ).sqrt()
    }

    pub fn normalize(&self) -> Entity {
        let m: f64 = self.magnitude(); 
        Entity::create(
            self.x() / m, 
            self.y() / m,
            self.z() / m, 
            self.w() / m
        )
    }    
}

impl PartialEq for Entity {
    fn eq(&self, rhs: &Entity) -> bool {
        fuzzy_equal(self.x(), rhs.x()) &&
        fuzzy_equal(self.y(), rhs.y()) &&
        fuzzy_equal(self.z(), rhs.z()) &&         
        fuzzy_equal(self.w(), rhs.w())  
    }
}

impl PartialEq<Point> for Entity {
    fn eq(&self, rhs: &Point) -> bool {
        *self == rhs.entity
    }
}

impl PartialEq<Vector> for Entity {
    fn eq(&self, rhs: &Vector) -> bool {
        *self == rhs.entity
    }
}

impl Add for Entity {
    type Output = Self; 

    fn add(self, rhs: Self) -> Self {
        Self {
            values: (
                self.x() + rhs.x(), 
                self.y() + rhs.y(),
                self.z() + rhs.z(),
                self.w() + rhs.w()
            )
        }
    }
}

impl Sub for Entity {
    type Output = Self; 

    fn sub(self, rhs: Self) -> Self {
        Self {
            values: (
                self.x() - rhs.x(), 
                self.y() - rhs.y(),
                self.z() - rhs.z(),
                self.w() - rhs.w()
            )
        }
    }
}

impl Neg for Entity {
    type Output = Self; 

    fn neg(self) -> Self {
        Entity::create(-self.x(), -self.y(), -self.z(), -self.w())
    }
}

impl Mul<f64> for Entity {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Entity::create(
            self.x() * rhs, 
            self.y() * rhs, 
            self.z() * rhs,
            self.w() * rhs 
        )
    }
}

impl Div<f64> for Entity {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Entity::create(
            self.x() / rhs, 
            self.y() / rhs, 
            self.z() / rhs,
            self.w() / rhs
        )
    }
}

impl Coordinate for Entity {
    fn x(&self) -> f64 {
        self.values.0
    }

    fn y(&self) -> f64 {
        self.values.1
    }

    fn z(&self) -> f64 {
        self.values.2
    }

    fn w(&self) -> f64 {
        self.values.3
    }
}

pub fn entity_dot (a: &Entity, b: &Entity) -> f64 {
    ( a.x() * b.x() ) + 
    ( a.y() * b.y() ) + 
    ( a.z() * b.z() ) + 
    ( a.w() * b.w() )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_is_a_point() {
        let a: Entity = Entity::create(4.3, -4.2, 3.1, 1.0 );
        assert_eq!(a.x(), 4.3);
        assert_eq!(a.y(), -4.2);
        assert_eq!(a.z(), 3.1);
        assert_eq!(a.w(), 1.0);
        assert_eq!(Point::create(4.3, -4.2, 3.1), a)
    }

    #[test]
    fn tuples_is_a_vector() {
        let a: Entity = Entity::create(4.3, -4.2, 3.1, 0.0);
        assert_eq!(a.x(), 4.3);
        assert_eq!(a.y(), -4.2);
        assert_eq!(a.z(), 3.1);
        assert_eq!(a.w(), 0.0);
        assert_eq!(Vector::create(4.3, -4.2, 3.1), a)
    }

    #[test]
    fn adding_two_tuples() {
        let e1: Entity = Entity::create(3.0, -2.0, 5.0, 1.0);
        let e2: Entity = Entity::create(-2.0, 3.0, 1.0, 0.0);
        assert_eq!(e1 + e2, Entity::create(1.0, 1.0, 6.0, 1.0));
    }


    #[test]
    fn negating_a_tuple() {
        let e: Entity = Entity::create( 1_f64, 2_f64, 3_f64, 4_f64 ); 
        assert_eq!(-e, Entity::create(-1_f64, -2_f64, -3_f64, -4_f64)); 
    }

    #[test]
    fn multiplying_a_tuple_by_a_scalar() {
        let a: Entity = Entity::create(1., -2_f64, 3_f64, -4_f64); 
        assert_eq!(a * 3.5, Entity::create(3.5, -7.0, 10.5, -14.0));
    }

    // TODO: impl Mul<Entity> for f64
    // #[test]
    // fn multiplying_a_scalar_by_a_tuple() {
    //     let a: Entity = Entity::create(1., -2.0, 3.0); 
    //     assert_eq!(3.5 * a, Entity::create(3.5, -7.0, 10.5));
    // }

    #[test]
    fn multiplying_a_tuple_by_a_fraction() {
        let a: Entity = Entity::create(1., -2.0, 3.0, -4.0); 
        assert_eq!(a * 0.5, Entity::create(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn dividing_a_tuple_by_a_scalar() {
        let a: Entity = Entity::create(1.0, -2.0, 3.0, -4_f64); 
        assert_eq!(a / 2.0, Entity::create(0.5, -1.0, 1.5, -2.0));
    }

    #[test] 
    fn dot_product_of_two_tuples() {
        let a: Entity = Entity::create(1_f64, 2_f64, 3_f64, 0_f64);
        let b: Entity = Entity::create(2_f64, 3_f64, 4_f64, 0_f64);
        assert_eq!(entity_dot(&a, &b), 20_f64);
    }

}