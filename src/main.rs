use macroquad::prelude::*;
use crate::data_reader::load_planetary_data;
use crate::physics::{update_bodies, PhysObject};

pub mod physics;
mod data_reader;

fn simulation_to_screen_coordinates(body_coords : [f64;2], screen_center : [f32;2], space_factor : f64, display_size : [f32;2]) ->[f32;2]{
    [
        (body_coords[0]/space_factor) as f32 + (display_size[0]/3.) + screen_center[0],
        (body_coords[1]/space_factor) as f32 + (display_size[1]/3.) + screen_center[1]
    ]
}

#[macroquad::main("SolarSystem")]
async fn main() {
    let dt= 86400.;
    let space_factor = 150e7;
    let mut bodies = load_planetary_data().unwrap();
    let sun = PhysObject{
        body_name: String::from("Sun"),
        pos: [0.0,0.0],
        vel: [0.0,0.0],
        mass: 1.988416e30,
    };
    bodies.push(sun);

    let mut screen_center : [f32;2] = [0f32;2];

    loop{
        clear_background(BLACK);

        for body in &bodies{
            let body_screen_coords = simulation_to_screen_coordinates(body.pos, [0.,0.],space_factor,[1920.,1080.]);
            draw_circle(body_screen_coords[0], body_screen_coords[1], 2., WHITE);
        }
        update_bodies(& mut bodies, dt);
        next_frame().await
    }
}