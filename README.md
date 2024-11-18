[Ver el video de funcionamiento aquí](https://www.canva.com/design/DAGWzliJwSA/m_FdMVfgiOaKfPgULj7OHQ/watch?utm_content=DAGWzliJwSA&utm_campaign=designshare&utm_medium=link&utm_source=editor)  

🌌 Sistema Solar Simulado
Este proyecto es una simulación interactiva de un sistema solar creado en Rust, utilizando un framebuffer personalizado para renderizar gráficos. Incluye planetas, órbitas, una nave espacial y un skybox lleno de estrellas que se mueven en el fondo.

🚀 Características
Sistema Solar:

Incluye varios planetas con órbitas circulares representadas visualmente.
Los planetas se mueven en trayectorias simuladas alrededor de una estrella central.
Skybox:

Un fondo de estrellas dinámico que se genera aleatoriamente y aparece detrás de los planetas y las órbitas.
Interacción:

Nave espacial que puede moverse en el sistema.
Soporte para cambiar entre diferentes vistas de planetas y elementos celestiales.
Órbitas Renderizadas:

Las órbitas de los planetas son dibujadas con líneas suaves.

📂 Estructura del Proyecto

├── assets/
│   ├── nave.obj          # Modelo de la nave espacial.
│   ├── sphere.obj        # Modelo base de los planetas.
│   ├── textures/         # (Opcional) Texturas futuras.
├── src/
│   ├── main.rs           # Punto de entrada del programa.
│   ├── framebuffer.rs    # Manejador del framebuffer para dibujar.
│   ├── shaders.rs        # Sombras personalizadas para planetas y estrellas.
│   ├── planet.rs         # Lógica de los planetas y sus propiedades.
│   ├── orbit.rs          # Lógica para renderizar órbitas.
│   ├── camera.rs         # Control de la cámara en el espacio 3D.
├── Cargo.toml            # Dependencias del proyecto.

⚙️ Requisitos del Sistema
Lenguaje: Rust (1.70+)
Dependencias:
minifb: Para la creación de ventanas y renderizado.
nalgebra: Para cálculos matemáticos en 3D.
Sistema Operativo: Funciona en Windows, macOS y Linux.

git clone https://github.com/tu-usuario/sistema-solar-simulado.git
cd sistema-solar-simulado

Instala las dependencias:
cargo build --release

Ejecuta el programa:
cargo run --release

🕹️ Controles
Mover la cámara: Flecha arriba, abajo, izquierda, derecha
Salir del programa:
Escape: Cierra la ventana.

📜 Créditos
Este proyecto fue desarrollado como parte de una práctica en Rust para aprender sobre gráficos, simulaciones y sistemas solares.
Autor: Michelle Mejia
