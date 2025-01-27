use macroquad::prelude::*;
use crate::data_reader::load_planetary_data;
use crate::physics::{update_bodies, PhysObject};
use crate::render_config::PLANET_CONFIG;

pub mod physics;
mod data_reader;
mod render_config;

const CAMERA_PAN_SPEED : f32 = 20.0;

fn simulation_to_screen_coordinates(body_coords : [f64;2], screen_center : [f32;2], space_factor : f64, display_size : [f32;2]) ->[f32;2]{
    [
        (body_coords[0]/space_factor) as f32 + (display_size[0]/3.) + screen_center[0],
        (body_coords[1]/space_factor) as f32 + (display_size[1]/3.) + screen_center[1]
    ]
}

#[macroquad::main("SolarSystem")]
async fn main() {
    let dt= 86400.;
    let mut iterations_per_frame = 1.;
    let mut space_factor = 150e7;
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
        screen_center = handle_camera(screen_center);
        iterations_per_frame = handle_time_scaling_input(iterations_per_frame);
        space_factor = handle_space_scaling_input(space_factor);
        clear_background(BLACK);
        for body in &bodies{
            let draw_config = PLANET_CONFIG.get(&body.body_name as &str);
            let color = draw_config.unwrap().color;
            let radius = draw_config.unwrap().radius;
            let body_screen_coords = simulation_to_screen_coordinates(body.pos, screen_center, space_factor,[1920.,1080.]);
            draw_circle(body_screen_coords[0], body_screen_coords[1], radius, color);
        }
        if iterations_per_frame<1.{
            update_bodies(& mut bodies, dt*iterations_per_frame);
        }else{
            for _i in 0..iterations_per_frame.round() as i64{
                update_bodies(& mut bodies, dt);
            }
        }
        next_frame().await
    }
}

fn handle_camera(screen_center : [f32;2]) -> [f32;2]{
    let mut new_screen_center = screen_center;
    if is_key_down(KeyCode::Up) {
        new_screen_center = [new_screen_center[0], new_screen_center[1]+CAMERA_PAN_SPEED];
    }
    if is_key_down(KeyCode::Down) {
        new_screen_center = [new_screen_center[0], new_screen_center[1]-CAMERA_PAN_SPEED];
    }
    if is_key_down(KeyCode::Left) {
        new_screen_center = [new_screen_center[0]+CAMERA_PAN_SPEED, new_screen_center[1]];
    }
    if is_key_down(KeyCode::Right){
        new_screen_center = [new_screen_center[0]-CAMERA_PAN_SPEED, new_screen_center[1]];
    }
    new_screen_center
}

fn handle_time_scaling_input(iterations_per_frame : f64)->f64{
    if is_key_pressed(KeyCode::Comma){
        return iterations_per_frame*0.5;
    }
    if is_key_pressed(KeyCode::Period){
        return iterations_per_frame*2.;
    }
    iterations_per_frame
}

fn handle_space_scaling_input(space_factor : f64)->f64{
    if is_key_pressed(KeyCode::KpAdd){
        return space_factor*0.5;
    }
    if is_key_pressed(KeyCode::KpSubtract){
        return space_factor*2.;
    }
    space_factor
}
