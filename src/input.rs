use crate::flatlander::Flatlander;

pub fn leer_theta_and_n() -> (f64, i32) {
    (45., 3)
}

pub fn leer_flatlanders(n: i32) -> Vec<Flatlander> {
    let v = vec![Flatlander(32, 50), Flatlander(3, 20)];

    v
}
