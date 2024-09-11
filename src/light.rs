use cgmath::Vector3;
use crate::light::LightType::Point;

enum LightType {
    Point,
    Directional,
    Ambient
}
struct Light {
    kind: LightType,
    pos_or_direction: Vector3<f64>,
    intensity: f64
}


impl Light {
    fn new(kind: LightType, pos_or_direction: Vector3<f64>, intensity: f64) -> Light {
        Self {
            kind,
            pos_or_direction,
            intensity
        }
    }

    fn default() -> Light {
        Self {
            kind: Point,
            pos_or_direction: Vector3{x:0.0,y:0.0,z:0.0},
            intensity: 1.0
        }
    }
}
