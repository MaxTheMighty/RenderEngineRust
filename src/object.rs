use cgmath::Vector3;

pub trait Normal {
    //Normal at point
    fn normal(&self, point: &Vector3<f64>) -> Vector3<f64>;
    //
}

pub trait Object {
    fn ray_intersections(&self, ray_origin: &Vector3<f64>, ray_direction: &Vector3<f64>) -> (f64, f64);
}