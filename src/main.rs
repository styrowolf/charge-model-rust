extern crate vector;
use vector::{Vector, angle};

fn main() {
    let q1 = PointCharge::new(0.0, 0.3, 2.0e-6);
    let q2 = PointCharge::new(0.0, 0.0, -4.0e-6);
    let q3 = PointCharge::new(0.4, 0.0, 4.0e-6);
    let f_net = q3.net_force(&[q1, q2]);
    println!("{:#?}", f_net);
    println!("{:#?}", f_net.angle.to_degrees());
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Coordinates {
    x: f64,
    y: f64,
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct PointCharge {
    c: Coordinates,
    q: Charge,
}

impl PointCharge {
    fn force(&self, other: &PointCharge) -> Vector {
        let delta_x = other.c.x - self.c.x;
        let delta_y = other.c.y - self.c.y;

        let angle = angle(delta_x, delta_y);
        let distance = f64::hypot(delta_x, delta_y);

        let force = coulombs_law(self.q.magnitude, other.q.magnitude, distance);
        match self.interaction(other) {
            Interaction::Attract => Vector::new_with_radians(force, angle),
            Interaction::Dispel => Vector::new_with_radians(force, angle).recip(),
        }
    }

    fn interaction(&self, other: &Self) -> Interaction {
        self.q.interaction(&other.q)
    }

    fn net_force(&self, charges: &[PointCharge]) -> Vector {
        let mut f_net = Vector::new_with_radians(0.0, 0.0);
        for c in charges {
            f_net += self.force(c);
        }
        f_net
    }

    fn new(x: f64, y: f64, q: f64) -> Self {
        PointCharge {
            c: Coordinates { x: x, y: y},
            q: Charge { magnitude: q.abs(), sign: (match q.is_sign_positive() {
                true => Sign::Positive,
                false => Sign::Negative,
            }) }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Charge {
    magnitude: f64,
    sign: Sign
}

impl Charge {
    fn interaction(&self, other: &Self) -> Interaction {
        match self.sign == other.sign {
            true => Interaction::Dispel,
            false => Interaction::Attract,
        }
    }
}
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Sign {
    Positive,
    Negative,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Interaction {
    Attract,
    Dispel
}

fn coulombs_law(q1: f64, q2: f64, d: f64) -> f64 {
    let k: f64 = 8.99e9;
    k * q1 * q2 / d.powi(2)
}