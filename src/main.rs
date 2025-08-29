mod errors;
mod flatlander;
mod input;
mod solve;

use crate::errors::Error;
use crate::input::parsear_lineas;
use crate::solve::Solver;

/// Esta funcion matchea el error recibido e imprime un string por stderr segun corresponda.
fn handle_error(e: Error) {
    match e {
        Error::IO => eprintln!("Error: IO"),
        Error::FueraRango => eprintln!("Error: Fuera de rango"),
        Error::ValorFaltante => eprintln!("Error: Valor faltante"),
        Error::NumeroInvalido => eprintln!("Error: Numero invalido"),
        Error::LineaFaltante => eprintln!("Error: Linea faltante"),
    };
}

fn main() {
    let (theta, flatlanders) = match parsear_lineas() {
        Ok(s) => s,
        Err(e) => {
            handle_error(e);
            return;
        }
    };

    let mut solver: Solver = Solver::new(flatlanders, theta);

    let resultado = solver.solve();

    println!("{:.6}", resultado); //medio en veremos esto
}
