use macroquad::prelude::*;
use ::phf::{Map, phf_map};

pub struct RenderConfigEntry {
    pub radius: f32,
    pub color: Color,
}

pub static PLANET_CONFIG: Map<&str, RenderConfigEntry> = phf_map!{
    "Sun" => RenderConfigEntry{
        radius: 3.,
        color: YELLOW
    },
    "Mercury" => RenderConfigEntry{
        radius: 3.,
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
        radius: 3.,
        color: RED
    },
    "Jupiter" => RenderConfigEntry{
        radius: 3.,
        color: ORANGE
    },
    "Saturn" => RenderConfigEntry{
        radius: 3.,
        color: GOLD
    },
    "Uranus" => RenderConfigEntry{
        radius: 3.,
        color: DARKGREEN
    },
    "Neptune" => RenderConfigEntry{
        radius: 3.,
        color: BLUE
    },


};
