#![cfg_attr(debug_assertions, allow(unused_imports))]

use rust_raytracer::coordinate::{ point::Point, vector::Vector, vector::normalize };
use rust_raytracer::exercises::ch1:: { Environment, Projectile, simulate, tick }; 

fn main() {
    let p: Projectile = Projectile { 
        position: Point::create(0_f64, 1_f64, 0_f64),
        velocity: normalize(&Vector::create(1_f64, 1_f64, 0_f64))
    };
    let e: Environment = Environment {
        gravity: Vector::create(0_f64, -0.1_f64, 0_f64),
        wind: Vector::create(-0.01_f64, 0_f64, 0_f64)
    }; 

    simulate(&p, &e);
}
