pub struct PhysObject{
    pub body_name:String,
    pub pos: [f64;2],
    pub vel: [f64;2],
    pub mass: f64,
}

const GRAVITATIONAL_CONSTANT : f64 = 6.6743e-11;

pub fn get_gravitational_force(a : &PhysObject, b : &PhysObject) -> f64 {
    let distance = distance(a,b);
    (GRAVITATIONAL_CONSTANT*a.mass*b.mass)/(distance.powi(2))
}

pub fn distance(a : &PhysObject, b : &PhysObject) -> f64 {
    ((a.pos[0]-b.pos[0]).powi(2) + (a.pos[1]-b.pos[1]).powi(2)).sqrt()
}

fn vector_length(vector : [f64;2])->f64{
    (vector[0]*vector[0] + vector[1]*vector[1]).sqrt()
}

fn normalize_vector(vector : [f64;2])-> [f64;2]{
    let vector_length = vector_length(vector);
    if vector[0]==0. && vector[1]==0.{
        return vector;
    }
    [vector[0]/vector_length, vector[1]/vector_length]
}

impl PhysObject{
    fn apply_force(&mut self, force : [f64;2]){
        self.vel[0] += force[0]/self.mass;
        self.vel[1] += force[1]/self.mass;
    }

    fn update(&mut self, dtime : f64){
        self.pos[0] += self.vel[0]*dtime;
        self.pos[1] += self.vel[1]*dtime;
    }
}

pub fn update_bodies(bodies: &mut [PhysObject], dt: f64){

    let forces = get_forces_for_bodies(bodies, dt);
    apply_forces(bodies, &*forces);
    for body in bodies{
        body.update(dt);
    }
}

fn get_forces_for_bodies(bodies: &[PhysObject], dt: f64)-> Vec<f64>{

    let no_bodies = bodies.len();
    let mut forces = vec![0.0; no_bodies*2];
    let mut j = 0;
    for body1 in bodies{
        for body2 in bodies{
            if body1.body_name == body2.body_name{
                j+=1;
                continue;
            }

            let force : f64 = get_gravitational_force(body1, body2) * dt;
            let mut direction = [
                body1.pos[0]-body2.pos[0],
                body1.pos[1]-body2.pos[1]
            ];

            direction = normalize_vector(direction);
            forces[2*j]+=force*direction[0];
            forces[2*j+1]+=force*direction[1];
            j+=1;
        }
        j=0;
    }
    forces
}

fn apply_forces(bodies: &mut [PhysObject], forces: &[f64]){
    for i in 0..bodies.len(){
        bodies[i].apply_force([forces[2*i],forces[2*i+1]]);
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let obj1 = PhysObject{
            body_name: String::from("Body1"),
            pos: [0.0,0.0],
            vel: [0.0,0.0],
            mass: 5.9722e24,
        };

        let obj2 = PhysObject{
            body_name: String::from("Body2"),
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
            body_name: String::from("Body1"),
            pos: [0.0, 1.495978707e11],
            vel: [0.0,0.0],
            mass: 5.9722e24,
        };

        let obj2 = PhysObject{
            body_name: String::from("Body2"),
            pos: [0.0,0.0],
            vel: [0.0,0.0],
            mass: 1.988416e30,
        };

        let force = get_gravitational_force(&obj1, &obj2);
        let expected_force = 3.54e22;
        assert!((force - expected_force).abs()/expected_force < 0.001);
    }

    #[test]
    fn test_vector_length() {
        let vector1 = [0.0,0.0];
        let vector2 = [0.0,1.0];
        let vector3 = [3.0,3.0];
        let vector_length1 = vector_length(vector1);
        let vector_length2 = vector_length(vector2);
        let vector_length3 = vector_length(vector3);

        let expected_length1 = 0.0;
        let expected_length2 = 1.0;
        let expected_length3 = 4.24264;

        assert!((vector_length1-expected_length1).abs() < 0.01);
        assert!((vector_length2-expected_length2).abs() < 0.01);
        assert!((vector_length3-expected_length3).abs() < 0.01);
    }

    #[test]
    fn test_get_forces(){
        let obj1 = PhysObject{
            body_name: String::from("Earth"),
            pos: [0.0, 1.495978707e11],
            vel: [0.0,0.0],
            mass: 5.9722e24,
        };

        let obj2 = PhysObject{
            body_name: String::from("Sun"),
            pos: [0.0,0.0],
            vel: [0.0,0.0],
            mass: 1.988416e30,
        };
        let dt = 0.1;
        let bodies = [obj1, obj2];
        let forces = get_forces_for_bodies(&bodies, dt);
        let expected_forces = vec![0.,-3.5415753e22*dt, 0., 3.5415753e22*dt];
        assert!((forces[0]-expected_forces[0]).abs() < 1e14);
        assert!((forces[1]-expected_forces[1]).abs() < 1e14);
        assert!((forces[2]-expected_forces[2]).abs() < 1e14);
        assert!((forces[3]-expected_forces[3]).abs() < 1e14);
    }

    #[test]
    fn test_normalize_vector(){
        let vector1 = [0.0,0.0];
        let vector2 = [0.0,1.0];
        let vector3 = [3.0,3.0];

        let normalized1 = normalize_vector(vector1);
        let normalized2 = normalize_vector(vector2);
        let normalized3 = normalize_vector(vector3);

        assert!((normalized1[0]- 0.).abs()<0.0001);
        assert!((normalized1[1]- 0.).abs()<0.0001);
        assert!((normalized2[0]- 0.).abs()<0.0001);
        assert!((normalized2[1]- 1.).abs()<0.0001);
        assert!((normalized3[0]- 1./(2_f64).sqrt()).abs()<0.0001);
        assert!((normalized3[1]- 1./(2_f64).sqrt()).abs()<0.0001);

    }
}