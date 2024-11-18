use crate::shaders::{vertex_shader, fragment_shader}; // Importa los shaders
use crate::framebuffer::Framebuffer;
use crate::vertex::Vertex;
use crate::Uniforms;
use nalgebra_glm::Vec3;

pub struct Planet {
    pub name: String,
    pub radius: f32,                // Tamaño del planeta
    pub orbit_radius: f32,          // Radio de la órbita
    pub orbit_speed: f32,           // Velocidad orbital
    pub rotation_speed: f32,        // Velocidad de rotación
    pub position: Vec3,             // Posición actual
    pub shader: fn(&crate::Fragment, &Uniforms) -> crate::Color, // Shader del planeta
}

impl Planet {
    pub fn update_position(&mut self, time: f32) {
        let angle = time * self.orbit_speed;
        self.position.x = self.orbit_radius * angle.cos();
        self.position.y = self.orbit_radius * angle.sin();
    }

    pub fn render(
        &self,
        framebuffer: &mut Framebuffer,
        vertex_array: &[Vertex],
        uniforms: &mut Uniforms,
        time: f32, // Agrega time como argumento
    ) {
        uniforms.model_matrix = crate::create_model_matrix(
            self.position,
            self.radius,
            Vec3::new(0.0, time * self.rotation_speed, 0.0), // Usa el argumento time
        );
    
        crate::render_celestial_body(
            framebuffer,
            vertex_array,
            uniforms,
            self.shader,
        );
    }
    
}
