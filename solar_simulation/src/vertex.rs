use nalgebra_glm::{Vec3, Vec2};
use crate::color::Color;

#[derive(Clone, Debug)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub tex_coords: Vec2,
    pub color: Color,
    pub transformed_position: Vec3,
    pub transformed_normal: Vec3,
}

impl Vertex {
    pub fn new(position: Vec3, normal: Vec3) -> Self {
        Vertex {
            position,
            normal,
            tex_coords: Vec2::new(0.0, 0.0),
            color: Color::new(255, 255, 255),
            transformed_position: Vec3::new(0.0, 0.0, 0.0),
            transformed_normal: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}
