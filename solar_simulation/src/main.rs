mod framebuffer;
mod fragment;
mod vertex;
mod color;
mod shaders;
mod triangle;


use nalgebra_glm::Vec3;
use framebuffer::Framebuffer;
use shaders::basic_shader;
use triangle::triangle;
use vertex::Vertex;
use fragment::Fragment;

pub struct Uniforms {
    pub time: f32,
}

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);
    let uniforms = Uniforms { time: 0.0 };

    let vertex1 = Vertex::new(Vec3::new(100.0, 100.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
    let vertex2 = Vertex::new(Vec3::new(400.0, 100.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
    let vertex3 = Vertex::new(Vec3::new(250.0, 400.0, 0.0), Vec3::new(0.0, 0.0, 1.0));

    let fragments = triangle(&vertex1, &vertex2, &vertex3);

    for fragment in fragments {
        let color = basic_shader(&fragment, &uniforms).to_hex();
        framebuffer.set_current_color(color);
        framebuffer.point(fragment.position.x as usize, fragment.position.y as usize, 0.0);
    }

    // Guardar el framebuffer en una imagen o mostrarlo en una ventana...
}
