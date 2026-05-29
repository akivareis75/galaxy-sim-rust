use macroquad::prelude::*;

const G: f32 = 450.0;
const SOFTENING: f32 = 20.0; 

struct Body {
    pos: Vec2,
    vel: Vec2,
    mass: f32,
    color: Color,
    is_star: bool,
}

impl Body {
    /// Cria uma estrela dentro de um volume esférico com velocidade orbital ZERO
    fn new_spherical_star(center: Vec2, radius: f32, galaxy_vel: Vec2, color: Color) -> Self {
        let angle = rand::gen_range(0.0, std::f32::consts::TAU);
        let r = rand::gen_range(0.0, radius);
        let pos = center + Vec2::new(angle.cos(), angle.sin()) * r;

        Self {
            pos,
            vel: galaxy_vel,
            mass: 0.1,
            color,
            is_star: true,
        }
    }
}

/// Função auxiliar para evitar código repetido ao criar múltiplas galáxias
fn spawn_galaxy(
    bodies: &mut Vec<Body>, 
    center: Vec2, 
    vel: Vec2, 
    core_mass: f32, 
    num_stars: usize, 
    core_color: Color, 
    star_color: Color
) {
    // Adiciona o núcleo supermassivo
    bodies.push(Body { pos: center, vel, mass: core_mass, color: core_color, is_star: false });
    
    // Adiciona as estrelas
    for _ in 0..num_stars {
        bodies.push(Body::new_spherical_star(center, 100.0, vel, star_color));
    }
}

#[macroquad::main("Colapso Gravitacional: 3 Galáxias")]
async fn main() {
    let mut bodies = Vec::new();
    
    // Pegamos as dimensões assim que a janela abre
    let sw = screen_width();
    let sh = screen_height();
    let screen_center = Vec2::new(sw * 0.5, sh * 0.5);
    
    // Configurações gerais da simulação
    let spawn_radius = sh * 0.35; // Distância do centro
    let speed = 0.6;              // Velocidade inicial de aproximação
    let num_stars = 600;

    // --- Galáxia 1 (Topo) ---
    let angle1 = -std::f32::consts::FRAC_PI_2; // -90 graus
    let c1 = screen_center + Vec2::new(angle1.cos(), angle1.sin()) * spawn_radius;
    let v1 = (screen_center - c1).normalize() * speed; // Vetor apontando pro centro
    spawn_galaxy(&mut bodies, c1, v1, 1500.0, num_stars, BLUE, SKYBLUE);

    // --- Galáxia 2 (Inferior Esquerda) ---
    let angle2 = angle1 + (2.0 * std::f32::consts::PI / 3.0); // +120 graus
    let c2 = screen_center + Vec2::new(angle2.cos(), angle2.sin()) * spawn_radius;
    let v2 = (screen_center - c2).normalize() * speed;
    spawn_galaxy(&mut bodies, c2, v2, 1500.0, num_stars, ORANGE, GOLD);

    // --- Galáxia 3 (Inferior Direita) ---
    let angle3 = angle2 + (2.0 * std::f32::consts::PI / 3.0); // +120 graus
    let c3 = screen_center + Vec2::new(angle3.cos(), angle3.sin()) * spawn_radius;
    let v3 = (screen_center - c3).normalize() * speed;
    spawn_galaxy(&mut bodies, c3, v3, 1500.0, num_stars, GREEN, LIME);

    loop {
        clear_background(BLACK);
        let dt = get_frame_time().min(0.016);

        // Física N-Body: Interação apenas com os núcleos massivos (Módulos O(N*K) onde K=3)
        for i in 0..bodies.len() {
            let mut acc = Vec2::ZERO;
            let p_i = bodies[i].pos;

            for j in 0..bodies.len() {
                // Ignora auto-interação e a gravidade gerada por estrelas (apenas núcleos puxam)
                if i == j || bodies[j].is_star { continue; }

                let r = bodies[j].pos - p_i;
                let dist_sq = r.length_squared() + SOFTENING;
                let force_mag = G * bodies[j].mass / dist_sq;
                acc += r.normalize() * force_mag;
            }

            let v_current = bodies[i].vel;
            bodies[i].vel += acc * dt;
            bodies[i].pos += v_current * dt;
        }

        // Desenho
        for b in &bodies {
            let radius = if b.is_star { 1.2 } else { 5.0 };
            draw_circle(b.pos.x, b.pos.y, radius, b.color);
        }

        // UI
        draw_text("Colapso Gravitacional: 3 Galaxias", 20.0, 30.0, 20.0, WHITE);
        draw_text("Problema dos Tres Corpos (Caotico)", 20.0, 55.0, 16.0, GRAY);
        
        next_frame().await
    }
}