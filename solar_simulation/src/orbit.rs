use crate::framebuffer::Framebuffer;
use nalgebra_glm::Vec3;

pub fn render_orbit(framebuffer: &mut Framebuffer, orbit_radius: f32, center: Vec3, color: u32) {
    let num_segments = 100; // Resolución de la órbita (más segmentos = círculo más suave)
    let framebuffer_width = framebuffer.width as f32;
    let framebuffer_height = framebuffer.height as f32;

    // Calculamos el centro de la órbita en coordenadas del framebuffer
    let center_x = framebuffer_width / 2.0 + center.x;
    let center_y = framebuffer_height / 2.0 - center.y;

    // Calculamos el primer punto de la órbita
    let mut previous_point = (
        center_x + orbit_radius * theta(0).cos(),
        center_y + orbit_radius * theta(0).sin(),
    );

    for i in 1..=num_segments {
        let angle = 2.0 * std::f32::consts::PI * i as f32 / num_segments as f32;
        let next_point = (
            center_x + orbit_radius * angle.cos(),
            center_y + orbit_radius * angle.sin(),
        );

        // Convertimos los puntos en coordenadas enteras
        let x1 = previous_point.0 as usize;
        let y1 = previous_point.1 as usize;
        let x2 = next_point.0 as usize;
        let y2 = next_point.1 as usize;

        // Dibujamos la línea entre el punto actual y el siguiente
        framebuffer.draw_line(x1, y1, x2, y2, color);

        // Actualizamos el punto anterior
        previous_point = next_point;
    }
}

// Función auxiliar para calcular el ángulo (theta)
fn theta(segment: i32) -> f32 {
    2.0 * std::f32::consts::PI * segment as f32 / 100.0
}

