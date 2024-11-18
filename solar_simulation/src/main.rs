use nalgebra_glm::{Vec3, Mat4, look_at, perspective};
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use std::f32::consts::PI;
use crate::fragment::Fragment;
use crate::color::Color;

mod framebuffer;
mod triangle;
mod vertex;
mod obj;
mod color;
mod fragment;
mod shaders;
mod camera;

mod planet;
mod orbit;
use planet::Planet;
use orbit::render_orbit;

use framebuffer::Framebuffer;
use vertex::Vertex;
use obj::Obj;
use camera::Camera;
use triangle::triangle;
use shaders::{vertex_shader, fragment_shader, moon_shader, meteor_shader, ringed_planet_shader, earth_shader, gas_giant_shader, rocky_planet_shader, star_shader };
use fastnoise_lite::{FastNoiseLite, NoiseType, FractalType};

pub struct Uniforms {
    model_matrix: Mat4,
    view_matrix: Mat4,
    projection_matrix: Mat4,
    viewport_matrix: Mat4,
    time: u32,
    noise: FastNoiseLite
}

fn create_noise() -> FastNoiseLite {
    create_cloud_noise()
}

fn create_cloud_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(1337);
    noise.set_noise_type(Some(NoiseType::OpenSimplex2));
    noise
}

fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();

    let rotation_matrix_x = Mat4::new(
        1.0,  0.0,    0.0,   0.0,
        0.0,  cos_x, -sin_x, 0.0,
        0.0,  sin_x,  cos_x, 0.0,
        0.0,  0.0,    0.0,   1.0,
    );

    let rotation_matrix_y = Mat4::new(
        cos_y,  0.0,  sin_y, 0.0,
        0.0,    1.0,  0.0,   0.0,
        -sin_y, 0.0,  cos_y, 0.0,
        0.0,    0.0,  0.0,   1.0,
    );

    let rotation_matrix_z = Mat4::new(
        cos_z, -sin_z, 0.0, 0.0,
        sin_z,  cos_z, 0.0, 0.0,
        0.0,    0.0,  1.0, 0.0,
        0.0,    0.0,  0.0, 1.0,
    );

    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

    let transform_matrix = Mat4::new(
        scale, 0.0,   0.0,   translation.x,
        0.0,   scale, 0.0,   translation.y,
        0.0,   0.0,   scale, translation.z,
        0.0,   0.0,   0.0,   1.0,
    );

    transform_matrix * rotation_matrix
}


fn create_view_matrix(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    look_at(&eye, &center, &up)
}

fn create_perspective_matrix(window_width: f32, window_height: f32) -> Mat4 {
    let fov = 45.0 * PI / 180.0;
    let aspect_ratio = window_width / window_height;
    let near = 0.1;
    let far = 1000.0;

    perspective(fov, aspect_ratio, near, far)
}

fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0, 0.0, 0.0, width / 2.0,
        0.0, -height / 2.0, 0.0, height / 2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    // Vertex Shader
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    // Primitive Assembly
    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    // Rasterization
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    // Fragment Processing
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        if x < framebuffer.width && y < framebuffer.height {
            let shaded_color = fragment_shader(&fragment, &uniforms);
            let color = shaded_color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}

// Función general para renderizar un cuerpo celeste con su shader específico
fn render_celestial_body(
    framebuffer: &mut Framebuffer,
    vertex_array: &[Vertex],
    uniforms: &Uniforms,
    fragment_shader: fn(&Fragment, &Uniforms) -> Color,
) {
    // Vertex Shader
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = shaders::vertex_shader(vertex, uniforms); // Usamos el vertex_shader general
        transformed_vertices.push(transformed);
    }

    // Rasterización y procesado de fragmentos
    let mut fragments = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            let tri = [
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ];
            fragments.extend(triangle::triangle(&tri[0], &tri[1], &tri[2]));
        }
    }

    // Fragment Shader específico para cada planeta
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        if x < framebuffer.width && y < framebuffer.height {
            let color = fragment_shader(&fragment, &uniforms).to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}

fn apply_emissive_postprocess(framebuffer: &mut Framebuffer) {
    let width = framebuffer.width;
    let height = framebuffer.height;

    for y in 0..height {
        for x in 0..width {
            let index = y * width + x;
            let emissive_color = framebuffer.emissive_buffer[index];

            if emissive_color != 0 {
                // Expande el brillo a los píxeles vecinos
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;

                        if nx >= 0 && ny >= 0 && (nx as usize) < width && (ny as usize) < height {
                            let neighbor_index = ny as usize * width + nx as usize;
                            framebuffer.buffer[neighbor_index] = framebuffer.buffer[neighbor_index].saturating_add(emissive_color);
                        }
                    }
                }
            }
        }
    }
}

fn render_orbiting_moon(
    framebuffer: &mut Framebuffer,
    uniforms: &mut Uniforms,
    time: f32,
    orbit_radius: f32,
    planet_position: Vec3,
) {
    // Calcula la posición de la luna en su órbita
    let angle = time * 0.5; // Ajusta este valor para cambiar la velocidad de la órbita
    let moon_x = planet_position.x + orbit_radius * angle.cos();
    let moon_y = planet_position.y + orbit_radius * angle.sin();
    let moon_position = Vec3::new(moon_x, moon_y, planet_position.z);

    // Configura la matriz de modelo para la luna
    uniforms.model_matrix = create_model_matrix(moon_position, 0.1, Vec3::new(0.0, 0.0, 0.0));

    // Renderiza la luna usando el shader de luna
    //render_celestial_body(framebuffer, &get_moon_vertex_array(), uniforms, moon_shader);
}


fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 800;
    let framebuffer_height = 600;
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Sistema Solar Simulado",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    window.set_position(500, 500);
    window.update();

    framebuffer.set_background_color(0x060611);

    // Inicializar cámara
    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 10.0), // Posición inicial de la cámara
        Vec3::new(0.0, 0.0, 0.0), // Centro de atención
        Vec3::new(0.0, 1.0, 0.0), // Dirección 'up'
    );

    // Crear planetas
    let mut planets = vec![
        Planet {
            name: "Sol".to_string(),
            radius: 1.0,
            orbit_radius: 0.0, // El Sol no orbita
            orbit_speed: 0.0,
            rotation_speed: 0.01,
            position: Vec3::new(0.0, 0.0, 0.0),
            shader: star_shader,
        },
        Planet {
            name: "Mercurio".to_string(),
            radius: 0.2,
            orbit_radius: 2.0,
            orbit_speed: 1.5,
            rotation_speed: 0.1,
            position: Vec3::new(0.0, 0.0, 0.0),
            shader: rocky_planet_shader,
        },
        Planet {
            name: "Venus".to_string(),
            radius: 0.3,
            orbit_radius: 3.0,
            orbit_speed: 1.2,
            rotation_speed: 0.08,
            position: Vec3::new(0.0, 0.0, 0.0),
            shader: rocky_planet_shader,
        },
        Planet {
            name: "Tierra".to_string(),
            radius: 0.4,
            orbit_radius: 4.0,
            orbit_speed: 1.0,
            rotation_speed: 0.1,
            position: Vec3::new(0.0, 0.0, 0.0),
            shader: earth_shader,
        },
        Planet {
            name: "Marte".to_string(),
            radius: 0.3,
            orbit_radius: 5.0,
            orbit_speed: 0.8,
            rotation_speed: 0.09,
            position: Vec3::new(0.0, 0.0, 0.0),
            shader: rocky_planet_shader,
        },
        // Agregar más planetas si es necesario
    ];

    // Vertex array de una esfera (modelo básico para todos los planetas)
    let obj = Obj::load("assets/sphere-1.obj").expect("Failed to load obj");
    let vertex_arrays = obj.get_vertex_array();

    let mut time = 0.0;
    // Configuración de las órbitas
    let planet_orbits = vec![50.0, 100.0, 150.0, 200.0, 250.0]; // Radios de las órbitas
    let orbit_color = 0x353535; // Color blanco para las órbitas

    // Posiciones iniciales y velocidades angulares para los planetas
    let mut planet_angles: Vec<f32> = vec![0.0; planet_orbits.len()]; // Ángulos iniciales
    let angular_speeds: Vec<f32> = vec![0.01, 0.008, 0.006, 0.004, 0.002]; // Velocidades angulares

    // Bucle principal
    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        handle_input(&window, &mut camera);

        framebuffer.clear();

        let view_matrix = look_at(&camera.eye, &camera.center, &camera.up);
        let projection_matrix = perspective(
            45.0 * PI / 180.0,
            window_width as f32 / window_height as f32,
            0.1,
            1000.0,
        );
        let viewport_matrix = Mat4::new(
            framebuffer_width as f32 / 2.0, 0.0, 0.0, framebuffer_width as f32 / 2.0,
            0.0, framebuffer_height as f32 / 2.0, 0.0, framebuffer_height as f32 / 2.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );

        let noise = create_noise();

        // Uniforms comunes
        let mut uniforms = Uniforms {
            model_matrix: Mat4::identity(),
            view_matrix,
            projection_matrix,
            viewport_matrix,
            time: time as u32,
            noise,
        };
       

        
        // Dibuja las órbitas de los planetas
        for &orbit_radius in &planet_orbits {
            render_orbit(
                &mut framebuffer,
                orbit_radius,               // Radio de la órbita
                Vec3::new(0.0, 0.0, 0.0),   // Centro del sistema solar
                orbit_color,                // Color de la órbita
            );
        }

        
    
        // Actualizar y renderizar planetas
        for planet in &mut planets {
            planet.update_position(time);
            planet.render(&mut framebuffer, &vertex_arrays, &mut uniforms, time);
        }

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        time += 0.016; // Incrementa el tiempo para animaciones
        std::thread::sleep(frame_delay);
    }
}




fn handle_input(window: &Window, camera: &mut Camera) {
    let movement_speed = 1.0;
    let rotation_speed = PI/50.0;
    let zoom_speed = 0.1;
   
    //  camera orbit controls
    if window.is_key_down(Key::Left) {
      camera.orbit(rotation_speed, 0.0);
    }
    if window.is_key_down(Key::Right) {
      camera.orbit(-rotation_speed, 0.0);
    }
    if window.is_key_down(Key::W) {
      camera.orbit(0.0, -rotation_speed);
    }
    if window.is_key_down(Key::S) {
      camera.orbit(0.0, rotation_speed);
    }

    // Camera movement controls
    let mut movement = Vec3::new(0.0, 0.0, 0.0);
    if window.is_key_down(Key::A) {
      movement.x -= movement_speed;
    }
    if window.is_key_down(Key::D) {
      movement.x += movement_speed;
    }
    if window.is_key_down(Key::Q) {
      movement.y += movement_speed;
    }
    if window.is_key_down(Key::E) {
      movement.y -= movement_speed;
    }
    if movement.magnitude() > 0.0 {
      camera.move_center(movement);
    }

    // Camera zoom controls
    if window.is_key_down(Key::Up) {
      camera.zoom(zoom_speed);
    }
    if window.is_key_down(Key::Down) {
      camera.zoom(-zoom_speed);
    }
}
