use crate::framebuffer::Framebuffer;
use nalgebra_glm::Vec3;

pub fn render_orbit(framebuffer: &mut Framebuffer, orbit_radius: f32, center: Vec3) {
    let num_segments = 100;
    for i in 0..num_segments {
        let theta1 = 2.0 * std::f32::consts::PI * i as f32 / num_segments as f32;
        let theta2 = 2.0 * std::f32::consts::PI * (i + 1) as f32 / num_segments as f32;

        let x1 = center.x + orbit_radius * theta1.cos();
        let y1 = center.y + orbit_radius * theta1.sin();
        let x2 = center.x + orbit_radius * theta2.cos();
        let y2 = center.y + orbit_radius * theta2.sin();

        framebuffer.draw_line(
            x1 as usize,
            y1 as usize,
            x2 as usize,
            y2 as usize,
            0xFFFFFF, // Color blanco para las órbitas
        );
    }
}
