use std::io::Cursor;
use tp0_taller::errors::Error;
use tp0_taller::input::parsear_lineas;
use tp0_taller::solve::Solver;

#[test]
fn test_basico() {
    let input = b"30 3\n50 150\n0 100\n100 200\n";
    let mut cursor = Cursor::new(input);

    let (theta, flatlanders) = parsear_lineas(&mut cursor).unwrap();
    let mut solver = Solver::new(flatlanders, theta);
    let resultado = solver.solve();

    assert!((resultado - 446.4101615137755).abs() < 1e-9);
}

#[test]
fn test_error_rango() {
    let input = b"5 1\n 15 20"; //deberia dar fuera de rango (theta < 10)

    let mut cursor = Cursor::new(input);

    let res = parsear_lineas(&mut cursor);

    assert!(matches!(res, Err(Error::FueraRango)));
}

#[test]
fn test_error_linea_faltante() {
    let input = b"30 1\n"; //deberia dar linea faltante porque no hay flatlanders

    let mut cursor = Cursor::new(input);

    let res = parsear_lineas(&mut cursor);

    assert!(matches!(res, Err(Error::LineaFaltante)));
}

#[test]
fn test_valor_invalido() {
    let input = b"45 1\n -15 20"; //no se deberia poder parsear

    let mut cursor = Cursor::new(input);

    let res = parsear_lineas(&mut cursor);

    assert!(matches!(res, Err(Error::NumeroInvalido)));
}
