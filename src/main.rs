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
            vel: galaxy_vel, // A estrela apenas viaja junto com o núcleo
            mass: 0.1,
            color,
            is_star: true,
        }
    }
}

#[macroquad::main("Colapso Gravitacional: 2 Galáxias")]
async fn main() {
    let mut bodies = Vec::new();
    let sw = screen_width();
    let sh = screen_height();
    
    // Galáxia 1 (Esquerda) - Indo para o centro
    let c1 = Vec2::new(sw * 0.15, sh * 0.5);
    let v1 = Vec2::new(0.6, 0.0);
    bodies.push(Body { pos: c1, vel: v1, mass: 1500.0, color: BLUE, is_star: false });
    for _ in 0..600 {
        bodies.push(Body::new_spherical_star(c1, 100.0, v1, SKYBLUE));
    }

    // Galáxia 2 (Direita) - Indo para o centro
    let c2 = Vec2::new(sw * 0.85, sh * 0.5);
    let v2 = Vec2::new(-0.6, 0.0);
    bodies.push(Body { pos: c2, vel: v2, mass: 1500.0, color: ORANGE, is_star: false });
    for _ in 0..600 {
        bodies.push(Body::new_spherical_star(c2, 100.0, v2, GOLD));
    }

    loop {
        clear_background(BLACK);
        let dt = get_frame_time().min(0.016);

        // Física N-Body: Interação apenas com os núcleos massivos
        for i in 0..bodies.len() {
            let mut acc = Vec2::ZERO;
            let p_i = bodies[i].pos;

            for j in 0..bodies.len() {
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

        draw_text("Colapso Frontal (Momento Angular Nulo)", 20.0, 30.0, 20.0, WHITE);
        draw_text("Estrelas em queda livre radial", 20.0, 55.0, 16.0, GRAY);
        
        next_frame().await
    }
}