use crate::coordinate::{ entity::Entity, traits::Coordinate, vector::Vector };
use std::ops::{ Add, Sub };

#[derive(Debug, PartialEq)]
pub struct Point { pub entity: Entity }

impl Point {
    pub fn create(x: f64, y: f64, z: f64) -> Point {        
        Self {
            entity: Entity::create( x, y, z, 1_f64 )
        }    
    }

    pub fn create_from_entity(e: Entity) -> Point {
        Self {
            entity: e
        }
    }
}

impl PartialEq<Entity> for Point {
    fn eq(&self, rhs: &Entity) -> bool {
        self.entity == *rhs
    }
}

impl Clone for Point {
    fn clone(&self) -> Self {
        Point::create(self.x().clone(), self.y().clone(), self.z().clone())
    }
}

impl Add for Point {
    type Output = Vector;

    fn add(self, rhs: Self) -> Vector {
        Vector::create_from_entity(
            self.entity + rhs.entity
        )
    }
}

impl Add<Vector> for Point {
    type Output = Point; 

    fn add(self, rhs: Vector) -> Point {
        Point::create_from_entity(
            self.entity + rhs.entity
        )
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Vector {
        Vector::create_from_entity(
            self.entity - rhs.entity
        )
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Point {
        Point::create_from_entity(
            self.entity - rhs.entity
        )
    }
}

impl Coordinate for Point {
    fn x(&self) -> f64 {
        self.entity.x()
    }
    
    fn y(&self) -> f64 {
        self.entity.y()
    }

    fn z(&self) -> f64 {
        self.entity.z()
    }

    fn w(&self) -> f64 {
        self.entity.w()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_creates_tuple_with_w_1() {
        let p: Point = Point::create(4_f64, -4_f64, 3_f64);
        assert_eq!(p, Entity::create(4_f64, -4_f64, 3_f64, 1_f64))
    }
    
    #[test]
    fn subtracting_vector_from_point() {
        let p: Point = Point::create(3_f64, 2_f64, 1_f64); 
        let v: Vector = Vector::create(5_f64, 6_f64, 7_f64); 

        assert_eq!(p - v, Point::create(-2_f64, -4_f64, -6_f64)); 
    }

    #[test]
    fn subtracting_two_points_creates_vector() {
        let p1: Point = Point::create(3_f64, 2_f64, 1_f64); 
        let p2: Point = Point::create(5_f64, 6_f64, 7_f64); 

        assert_eq!(p1 - p2, Vector::create(-2_f64, -4_f64, -6_f64)); 
    }
    
}