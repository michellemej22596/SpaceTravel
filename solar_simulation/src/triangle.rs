use nalgebra_glm::{Vec3};
use crate::fragment::Fragment;
use crate::vertex::Vertex;

pub fn triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    // Rasterización por bounding box
    let min_x = v1.transformed_position.x.min(v2.transformed_position.x).min(v3.transformed_position.x) as i32;
    let max_x = v1.transformed_position.x.max(v2.transformed_position.x).max(v3.transformed_position.x) as i32;
    let min_y = v1.transformed_position.y.min(v2.transformed_position.y).min(v3.transformed_position.y) as i32;
    let max_y = v1.transformed_position.y.max(v2.transformed_position.y).max(v3.transformed_position.y) as i32;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let point = Vec3::new(x as f32 + 0.5, y as f32 + 0.5, 0.0);

            let edge1 = edge_function(&v1.transformed_position, &v2.transformed_position, &point);
            let edge2 = edge_function(&v2.transformed_position, &v3.transformed_position, &point);
            let edge3 = edge_function(&v3.transformed_position, &v1.transformed_position, &point);

            if edge1 >= 0.0 && edge2 >= 0.0 && edge3 >= 0.0 {
                fragments.push(Fragment::new(
                    x as f32,
                    y as f32,
                    v1.color, // Color del primer vértice (puedes interpolar colores aquí)
                    0.0,      // Profundidad por ahora fija
                    Vec3::new(0.0, 0.0, 1.0), // Normal arbitraria
                    1.0,      // Intensidad fija
                    point,    // Posición del fragmento
                ));
            }
        }
    }

    fragments
}

fn edge_function(v1: &Vec3, v2: &Vec3, point: &Vec3) -> f32 {
    (point.x - v1.x) * (v2.y - v1.y) - (point.y - v1.y) * (v2.x - v1.x)
}
