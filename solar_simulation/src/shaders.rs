use nalgebra_glm::{Vec3};
use crate::fragment::Fragment;
use crate::color::Color;
use crate::Uniforms;

// Shader básico: Devuelve un color sólido
pub fn basic_shader(fragment: &Fragment, _uniforms: &Uniforms) -> Color {
    Color::new(100, 100, 255) // Azul básico
}

// Shader para simular un planeta rocoso
pub fn rocky_planet_shader(fragment: &Fragment, _uniforms: &Uniforms) -> Color {
    let base_color = Color::new(139, 69, 19); // Marrón oscuro
    let variation = (fragment.vertex_position.x * 5.0).sin() * 0.5 + 0.5; // Variación basada en posición
    base_color * variation
}

// Shader para simular un gigante gaseoso
pub fn gas_giant_shader(fragment: &Fragment, _uniforms: &Uniforms) -> Color {
    let base_color = Color::new(255, 204, 153); // Color claro
    let variation = (fragment.vertex_position.y * 10.0).sin() * 0.5 + 0.5; // Variación en bandas
    base_color * variation
}

pub fn star_shader(fragment: &Fragment, _uniforms: &Uniforms) -> Color {
    let base_color = Color::new(255, 165, 0); // Naranja brillante
    let intensity: f32 = 1.0 - (fragment.vertex_position.x.powi(2) + fragment.vertex_position.y.powi(2)).sqrt() * 0.5;
    base_color * intensity.clamp(0.0, 1.0)
}
