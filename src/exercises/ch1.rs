use crate::coordinate::{ point::Point, traits::Coordinate, vector::Vector };
use std::fmt::{ Display, Formatter, Result};

#[derive(Debug)]
pub struct Projectile {
    pub position: Point, 
    pub velocity: Vector
}

impl Clone for Projectile {
    fn clone(&self) -> Projectile {
        Projectile {
            position: self.position.clone(),
            velocity: self.velocity.clone()
        }
    }
}

impl Display for Projectile {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f,"Proj - Position: x:{}, y:{}, z:{}", self.position.x(), self.position.y(), self.position.z())
    }
}

pub struct Environment {
    pub gravity: Vector, 
    pub wind: Vector
}

impl Clone for Environment {
    fn clone(&self) -> Environment {
        Environment {
            gravity: self.gravity.clone(),
            wind: self.wind.clone()
        }
    }
}

pub fn tick(e: &Environment, p: &Projectile) -> Projectile {
    let pos: Point = p.position.clone() + p.velocity.clone();
    let vel: Vector = p.velocity.clone() + e.gravity.clone() + e.wind.clone();
    Projectile {
        position: pos,
        velocity: vel
    }
}

pub fn simulate(p: &Projectile, e: &Environment) {
    let mut simulated_projectile: Projectile = p.clone();
    while simulated_projectile.position.y() > 0_f64 {
        println!("{}", simulated_projectile);
        simulated_projectile = tick(&e, &simulated_projectile); 
    }
}