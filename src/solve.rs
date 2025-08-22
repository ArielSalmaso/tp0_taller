use std::fmt::Error;

use crate::flatlander::Flatlander;

pub enum ArithError {
    TangenteEs0,
}

pub struct Solver {
    flatlanders: Vec<Flatlander>,
    tan_theta: f64,
}

fn calcular_tan(theta: f64) -> Result<f64, ArithError> {
    let theta_rads = theta.to_radians();
    let tan_theta = theta_rads.tan();

    if tan_theta == 0.0 {
        return Err(ArithError::TangenteEs0);
    }
    Ok(tan_theta)
}

impl Solver {
    pub fn new(flatlanders: Vec<Flatlander>, theta: f64) -> Result<Self, ArithError> {
        let tan_theta: f64 = calcular_tan(theta)?;
        Ok(Solver {
            flatlanders,
            tan_theta,
        })
    }

    pub fn solve(&self) -> f64 {
        //la idea acá es que haga un sort en los flatlanders por X, y despues vaya haciedn un while calculando las sombras de los que estan superpuestos.
        // En el while se ve si el siguiente esta supuersto y asi sucesivamente hasta llegar a uno que no lo esta. Se ahce el calculo de esa sombre y sigue hasta
        // Quedarse sin flatlanders. -> :Ok:

        return 42.;
    }
}
