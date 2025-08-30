use std::io;
use tp0_taller::errors::Error;
use tp0_taller::input::parsear_lineas;
use tp0_taller::solve::Solver;

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
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let (theta, flatlanders) = match parsear_lineas(&mut handle) {
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
