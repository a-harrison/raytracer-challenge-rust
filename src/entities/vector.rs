use crate::entities::{ entity::Entity, entity::entity_dot, traits::Coordinate };
use std::ops::{ Add, Sub };

#[derive(Debug, PartialEq)]
pub struct Vector { pub entity: Entity }

impl Vector {
    pub fn create(x: f64, y: f64, z: f64) -> Vector {        
        Self {
            entity: Entity::create( x, y, z, 0_f64 )
        }    
    }

    pub fn create_from_entity(e: Entity) -> Vector {
        Self {
            entity: e
        }
    }
}

impl Coordinate for Vector {
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

impl Clone for Vector {
    fn clone(&self) -> Self {
        Vector::create(self.x().clone(), self.y().clone(), self.z().clone())
    }
}

impl PartialEq<Entity> for Vector {
    fn eq(&self, rhs: &Entity) -> bool {
        self.entity == *rhs
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self {
        Vector::create_from_entity(
            self.entity + rhs.entity
        )
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vector::create_from_entity(
            self.entity - rhs.entity
        )
    }
}

pub fn magnitude(v: &Vector) -> f64 {
    v.entity.magnitude()
}

pub fn normalize(v: &Vector) -> Vector {
    Vector::create_from_entity(
        v.entity.normalize()
    )
}

pub fn dot(a: &Vector, b: &Vector) -> f64 {
    entity_dot(&a.entity, &b.entity)
}

pub fn cross(a: &Vector, b: &Vector) -> Vector {
    Vector::create(
        (a.y() * b.z()) - (a.z() * b.y()),
        (a.z() * b.x()) - (a.x() * b.z()), 
        (a.x() * b.y()) - (a.y() * b.x())
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn vector_creates_tuple_with_w_0() {
        let v: Vector = Vector::create(4_f64, -4_f64, 3_f64);
        assert_eq!(v, Entity::create(4_f64, -4_f64, 3_f64, 0_f64))
    }

    #[test]
    fn adding_two_vectors() {
        let v1: Vector = Vector::create(5_f64, 6_f64, 7_f64); 
        let v2: Vector = Vector::create(-2_f64, -4_f64, -6_f64); 

        assert_eq!(v1 + v2, Vector::create(3_f64, 2_f64, 1_f64)); 
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1: Vector = Vector::create(3_f64, 2_f64, 1_f64); 
        let v2: Vector = Vector::create(5_f64, 6_f64, 7_f64); 

        assert_eq!(v1 - v2, Vector::create(-2_f64, -4_f64, -6_f64)); 
    }

    #[test]
    fn subtracting_vector_from_zero_vector() {
        let zero: Vector = Vector::create(0_f64, 0_f64, 0_f64);
        let v: Vector = Vector::create(1_f64, -2_f64, 3_f64);

        assert_eq!(zero - v, Vector::create(-1_f64, 2_f64, -3_f64));
    }

    #[test]
    fn magnitude_of_100_vector() {
        let v = Vector::create(1_f64, 0_f64, 0_f64);
        assert_eq!(magnitude(&v), 1_f64);
    }

    
    #[test]
    fn magnitude_of_010_vector() {
        let v = Vector::create(0_f64, 1_f64, 0_f64);
        assert_eq!(magnitude(&v), 1_f64);
    }

    
    #[test]
    fn magnitude_of_001_vector() {
        let v = Vector::create(0_f64, 0_f64, 1_f64);
        assert_eq!(magnitude(&v), 1_f64);
    }

    
    #[test]
    fn magnitude_of_123_vector() {
        let v = Vector::create(1_f64, 2_f64, 3_f64);
        assert_eq!(magnitude(&v), 14_f64.sqrt());
    }

    
    #[test]
    fn magnitude_of_negated_123_vector() {
        let v = Vector::create(-1_f64, -2_f64, -3_f64);
        assert_eq!(magnitude(&v), 14_f64.sqrt());
    }

    #[test] 
    fn normalize_vector_400_gives_100() {
        let v = Vector::create(4_f64, 0_f64, 0_f64);
        assert_eq!(normalize(&v), Vector::create(1_f64, 0_f64, 0_f64));
    }
    
    #[test] 
    fn normalize_vector_123() {
        let v = Vector::create(1_f64, 2_f64, 3_f64);
        assert_eq!(normalize(&v), Vector::create(0.26726, 0.53452, 0.80178));
    }

    #[test] 
    fn magnitude_of_normalized_vector_is_1() {
        let v = Vector::create(1_f64, 2_f64, 3_f64);
        let norm = normalize(&v);
        assert_eq!(magnitude(&norm), 1_f64);
    }

    #[test] 
    fn dot_product_of_two_tuples() {
        let a: Vector = Vector::create(1_f64, 2_f64, 3_f64);
        let b: Vector = Vector::create(2_f64, 3_f64, 4_f64);
        assert_eq!(dot(&a, &b), 20_f64);
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let a: Vector = Vector::create(1_f64, 2_f64, 3_f64);
        let b: Vector = Vector::create(2_f64, 3_f64, 4_f64);
        assert_eq!(cross(&a,&b), Vector::create(-1_f64, 2_f64, -1_f64));
        assert_eq!(cross(&b,&a), Vector::create(1_f64, -2_f64, 1_f64));
    }
}