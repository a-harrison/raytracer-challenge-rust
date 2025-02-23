use crate::entities::{ entity::Entity, traits::Coordinate };
use std::ops::{ Add, Sub, Mul };

#[derive(Clone, Debug, PartialEq)]
pub struct Color {
    entity: Entity
}

impl Color {
    pub fn create(r: f64, g: f64, b: f64) -> Color {
        Self {
            entity: Entity::create(r, g, b, 0_f64)
        }
    }

    fn create_from_entity(e: Entity) -> Color {
        Self {
            entity: e
        }
    }

    fn r(&self) -> f64 {
        self.entity.x()    
    }
    
    fn g(&self) -> f64 {
        self.entity.y()    
    }

    fn b(&self) -> f64 {
        self.entity.z()    
    }

}

impl PartialEq<Entity> for Color {
    fn eq(&self, rhs: &Entity) -> bool {
        self.entity == *rhs 
    }
}

impl Add for Color {
    type Output = Color; 

    fn add(self, rhs: Self) -> Self {
        Color::create_from_entity(
            self.entity + rhs.entity
        )
    }
}

impl Sub for Color {
    type Output = Color; 

    fn sub(self, rhs: Self) -> Self {
        Color::create_from_entity(
            self.entity - rhs.entity
        )
    }
}

impl Mul for Color {
    type Output = Self; 

    fn mul(self, rhs: Self) -> Color {
        Color::create_from_entity(self.entity * rhs.entity)
    }
}

pub fn hadamard_product(c1: &Color, c2: &Color) -> Color {
    Color::create(
        c1.r() * c2.r(), 
        c1.g() * c2.g(),
         c1.b() * c2.b()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors_are_rgb_tuples() {
        let c: Color = Color::create(-0.5, 0.4, 1.7);
        assert_eq!(c.r(), -0.5_f64);
        assert_eq!(c.g(), 0.4_f64);
        assert_eq!(c.b(), 1.7_f64);
    }

    #[test]
    fn adding_colors() {
        let c1: Color = Color::create(0.9, 0.6, 0.75);
        let c2: Color = Color::create(0.7, 0.1, 0.25);

        assert_eq!(c1 + c2, Color::create(1.6, 0.7, 1.0));
    }

    #[test]
    fn subtracting_colors() {
        let c1: Color = Color::create(0.9, 0.6, 0.75);
        let c2: Color = Color::create(0.7, 0.1, 0.25);

        assert_eq!(c1 + c2, Color::create(1.6, 0.7, 1.0));
    }

    #[test]
    fn multiplying_colors() {
        let c1 : Color = Color::create(1.0, 0.2, 0.4);
        let c2: Color = Color::create(0.9, 1.0, 0.1); 

        assert_eq!(c1 * c2, Color::create(0.9, 0.2, 0.04));
    }

    #[test]
    fn multiplying_using_hadamard_product() {
        let c1 : Color = Color::create(1.0, 0.2, 0.4);
        let c2: Color = Color::create(0.9, 1.0, 0.1); 

        assert_eq!(hadamard_product(&c1, &c2), Color::create(0.9, 0.2, 0.04));
    }
}