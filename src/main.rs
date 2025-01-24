use macroquad::prelude::*;
use crate::data_reader::example;
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
    //println!("{:?}",example().err());
    let dt= 86400.;
    let space_factor = 150e7;
    let mut bodies = Vec::new();
    let obj1 = PhysObject{
        body_name: String::from("Earth"),
        pos: [0.0, 1.495978707e11],
        vel: [29.8e3,0.0],
        mass: 5.9722e24,
    };
    let obj2 = PhysObject{
        body_name: String::from("Sun"),
        pos: [0.0,0.0],
        vel: [0.0,0.0],
        mass: 1.988416e30,
    };
    bodies.push(obj1);
    bodies.push(obj2);

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