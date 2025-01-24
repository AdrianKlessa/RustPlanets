use std::{error::Error};
use serde::Deserialize;
use serde::de::{self, Deserializer};
use crate::physics::PhysObject;
#[derive(Debug, Deserialize)]
struct Record {
    #[serde(rename = "Planet")]
    planet: String,
    #[serde(rename = "Color")]
    color: String,
    #[serde(rename = "Mass (10^24kg)")]
    mass: f64,
    #[serde(rename = "Diameter (km)")]
    diameter: f64,
    #[serde(rename = "Density (kg/m^3)")]
    density: f64,
    #[serde(rename = "Surface Gravity(m/s^2)")]
    surface_gravity: f64,
    #[serde(rename = "Escape Velocity (km/s)")]
    escape_velocity: f64,
    #[serde(rename = "Rotation Period (hours)")]
    rotation_period: f64,
    #[serde(rename = "Length of Day (hours)")]
    length_of_day: f64,
    #[serde(rename = "Distance from Sun (10^6 km)")]
    distance_from_sun: f64,
    #[serde(rename = "Perihelion (10^6 km)")]
    perihelion: f64,
    #[serde(rename = "Aphelion (10^6 km)")]
    aphelion: f64,
    #[serde(rename = "Orbital Period (days)", deserialize_with = "parse_orbital_days")]
    orbital_period: f64,
    #[serde(rename = "Orbital Velocity (km/s)")]
    orbital_velocity: f64,
    #[serde(rename = "Orbital Inclination (degrees)")]
    orbital_inclination: f64,
    #[serde(rename = "Orbital Eccentricity")]
    orbital_eccentricity: f64,
    #[serde(rename = "Obliquity to Orbit (degrees)")]
    obliquity_to_orbit: f64,
    #[serde(rename = "Mean Temperature (C)")]
    mean_temperature: i32,
    #[serde(rename = "Surface Pressure (bars)")]
    surface_pressure: String, // Contains "Unknown" values
    #[serde(rename = "Number of Moons")]
    number_of_moons: u32,
    #[serde(rename = "Ring System?")]
    ring_system: String,
    #[serde(rename = "Global Magnetic Field?")]
    global_magnetic_field: String,
}
fn parse_orbital_days<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let s = s.replace(',', "");
    s.parse::<f64>().map_err(de::Error::custom)
}

fn record_to_planet(record : Record)->PhysObject{
    PhysObject{
        body_name: record.planet.to_string(),
        pos: [0.0, record.distance_from_sun*1e9],
        vel: [record.orbital_velocity*1e3, 0.0],
        mass: record.mass*1e24,
    }
}

pub fn load_planetary_data() -> Result<Vec<PhysObject>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("./data/solar_system_dataset.csv")?;
    let mut parsed = Vec::new();
    for result in rdr.deserialize(){
        let record: Record = result?;
        parsed.push(record_to_planet(record));
    }
    Ok(parsed)
}