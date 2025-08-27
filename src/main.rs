//TODO: cambiar los tipos de las variables a como pide el enunciado! y ver lo de las cotas de errores.
mod errors;
mod flatlander;
mod input;
mod solve;

use crate::errors::Error;
use crate::input::parsear_lineas;
use crate::solve::Solver;

fn handle_error(e: Error) {
    match e {
        Error::IO => eprintln!("Error: IO"),
        Error::FueraRango => eprintln!("Error: Fuera de rango"),
        Error::ValorFaltante => eprintln!("Error: Valor faltante"), //error de input
        Error::NumeroInvalido => eprintln!("Error: Numero invalido"),
        Error::LineaFaltante => eprintln!("Error: Linea faltante"),
    };
}

fn main() {}
