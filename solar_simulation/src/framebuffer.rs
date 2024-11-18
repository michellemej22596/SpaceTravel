
pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    pub zbuffer: Vec<f32>,
    pub emissive_buffer: Vec<u32>, // Nuevo buffer emisivo
    background_color: u32,
    current_color: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height],
            zbuffer: vec![f32::INFINITY; width * height],
            emissive_buffer: vec![0; width * height], // Inicializar con negro (sin emisión)
            background_color: 0x000000,
            current_color: 0xFFFFFF,
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = self.background_color;
        }
        for depth in self.zbuffer.iter_mut() {
            *depth = f32::INFINITY;
        }
        for emissive_pixel in self.emissive_buffer.iter_mut() {
            *emissive_pixel = 0; // Limpiar el buffer emisivo a negro
        }
    }

    pub fn draw_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, color: u32) {
        let dx = (x2 as isize - x1 as isize).abs() as usize;
        let dy = (y2 as isize - y1 as isize).abs() as usize;
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };

        let mut err = if dx > dy { dx as isize } else { -(dy as isize) } / 2;
        let mut current_x = x1 as isize;
        let mut current_y = y1 as isize;

        loop {
            // Dibuja el punto actual
            if current_x >= 0 && current_x < self.width as isize && current_y >= 0 && current_y < self.height as isize {
                self.buffer[current_y as usize * self.width + current_x as usize] = color;
            }

            // Salir si alcanzamos el punto final
            if current_x == x2 as isize && current_y == y2 as isize {
                break;
            }

            let e2 = err;
            if e2 > -(dx as isize) {
                err -= dy as isize;
                current_x += sx as isize;
            }
            if e2 < (dy as isize) {
                err += dx as isize;
                current_y += sy as isize;
            }
        }
    }

    pub fn point(&mut self, x: usize, y: usize, depth: f32) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;

            if self.zbuffer[index] > depth {
                self.buffer[index] = self.current_color;
                self.zbuffer[index] = depth;
            }
        }
    }

    // Método para escribir en el buffer emisivo
    pub fn emissive_point(&mut self, x: usize, y: usize, color: u32, depth: f32) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;

            if self.zbuffer[index] > depth {
                self.emissive_buffer[index] = color; // Guardar en el buffer emisivo
            }
        }
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }
}
