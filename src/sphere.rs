use cgmath::{InnerSpace, Vector3};
use crate::{color,object};

#[derive(Copy, Clone)]
pub struct Sphere{
    pub r: f64,
    pub origin: Vector3<f64>,
    pub color: color::Color,
    pub specular_reflection: f64
}

impl object::Normal for Sphere{

   fn normal(&self, point: &Vector3<f64>) -> Vector3<f64>{
        return (point - &self.origin).normalize();
    }

}

impl object::Object for Sphere {
     fn ray_intersections(&self, ray_origin: &Vector3<f64>, ray_direction: &Vector3<f64>) -> (f64, f64) {
        let co = ray_origin-&self.origin; //
        let a = ray_direction.dot(*ray_direction);
        let b = 2.0 * co.dot(*ray_direction);
        let c = (co.dot(co) - (self.r * self.r));
        let discriminant = (b * b - (4.0 * a * c));
        if(discriminant == 0.0){
            return (((-b - (discriminant).sqrt()) / 2.0*a), f64::INFINITY);
        }
        if discriminant < 0.0 {
            return (f64::INFINITY, f64::INFINITY);
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        return (t1, t2);
    }
}


fn main() {}