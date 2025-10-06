use nalgebra::Vector3;
use crate::color::Color;

#[derive(Clone)]
pub struct Light {
    pub position: Vector3<f32>, 
    pub color: Color,            
    pub intensity: f32,          
    pub radius: f32,             
}

impl Light {
    pub fn new(position: Vector3<f32>, color: Color, intensity: f32, radius: f32) -> Self {
        Light {
            position,
            color,
            intensity,
            radius,
        }
    }
}
