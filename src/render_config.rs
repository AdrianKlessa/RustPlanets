use macroquad::prelude::*;
use ::phf::{Map, phf_map};

pub struct RenderConfigEntry {
    pub radius: f32,
    pub color: Color,
}

pub static PLANET_CONFIG: Map<&str, RenderConfigEntry> = phf_map!{
    "Sun" => RenderConfigEntry{
        radius: 10.,
        color: YELLOW
    },
    "Mercury" => RenderConfigEntry{
        radius: 1.,
        color: GRAY
    },
    "Venus" => RenderConfigEntry{
        radius: 3.,
        color: BROWN
    },
    "Earth" => RenderConfigEntry{
        radius: 3.,
        color: SKYBLUE
    },
    "Mars" => RenderConfigEntry{
        radius: 2.,
        color: RED
    },
    "Jupiter" => RenderConfigEntry{
        radius: 7.,
        color: ORANGE
    },
    "Saturn" => RenderConfigEntry{
        radius: 7.,
        color: GOLD
    },
    "Uranus" => RenderConfigEntry{
        radius: 5.,
        color: BLUE
    },
    "Neptune" => RenderConfigEntry{
        radius: 5.,
        color: DARKBLUE
    },


};
