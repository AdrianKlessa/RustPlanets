pub struct PhysObject{
    pos: [f64;2],
    vel: [f64;2],
    mass: f64,
}

const GRAVITATIONAL_CONSTANT : f64 = 6.6743e-11;

pub fn get_gravitational_force(a : &PhysObject, b : &PhysObject) -> f64 {
    let distance = distance(a,b);
    (GRAVITATIONAL_CONSTANT*a.mass*b.mass)/(distance.powi(2))
}

pub fn distance(a : &PhysObject, b : &PhysObject) -> f64 {
    ((a.pos[0]-b.pos[0]).powi(2) + (a.pos[1]-b.pos[1]).powi(2)).sqrt()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let obj1 = PhysObject{
            pos: [0.0,0.0],
            vel: [0.0,0.0],
            mass: 5.9722e24,
        };

        let obj2 = PhysObject{
            pos: [2.5,2.5],
            vel: [0.0,0.0],
            mass: 1.988416e30,
        };

        let dist = distance(&obj1, &obj2);
        assert!((dist-3.53553).abs() < 0.01);
    }

    #[test]
    fn test_gravitational_force() {
        let obj1 = PhysObject{
            pos: [0.0, 1.495978707e11],
            vel: [0.0,0.0],
            mass: 5.9722e24,
        };

        let obj2 = PhysObject{
            pos: [0.0,0.0],
            vel: [0.0,0.0],
            mass: 1.988416e30,
        };

        let force = get_gravitational_force(&obj1, &obj2);
        let expected_force = 3.54e22;
        assert!((force - expected_force).abs()/expected_force < 0.001);
    }

}