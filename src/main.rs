//lee los datos, resulve y muestra el resultado

//TODO: cambiar los tipos de las variables a como pide el enunciado! y ver lo de las cotas de errores.
mod flatlander;
mod input;
mod solve;
mod errors;




use crate::input::*;
use crate::solve::Solver;
use crate::errors::Errors;

fn handle_error(e: Errors) {
    match e {
        Errors::IO => eprintln!("Error: IO"),
        Errors::FueraRango => eprintln!("Error: Fuera de rango"),
        Errors::ValorFaltante => eprintln!("Error: Valor faltante"), //error de input 
        Errors::NumeroInvalido => eprintln!("Error: Numero invalido"),
        Errors::LineaFaltante => eprintln!("Error: Linea faltante"),
    };

}

fn main() {
    // cambiar esto esta mal, lo deberia manejar acá
    let (theta, n) = leer_theta_and_n();
    let flatlanders = leer_flatlanders(n);

    // let solver: Solver = Solver::new(flatlanders, theta); 
    // println!("{}", solver.solve()) ver que hacer con esto
}
