//lee los datos, resulve y muestra el resultado

//TODO: cambiar los tipos de las variables a como pide el enunciado! y ver lo de las cotas de errores.
mod flatlander;
mod input;
mod solve;

use crate::input::*;
use crate::solve::{ArithError, Solver};

fn main() {
    // cambiar esto esta mal, lo deberia manejar acá
    let (theta, n) = leer_theta_and_n();
    let flatlanders = leer_flatlanders(n);

    let solver: Solver = match Solver::new(flatlanders, theta) {
        Ok(s) => s,
        Err(ArithError::TangenteEs0) => {
            println!(
                "Error al crear el solver, la tangente del angulo es 0. θ: {}",
                theta
            );
            return;
        }
    };
    solver.solve();
}
