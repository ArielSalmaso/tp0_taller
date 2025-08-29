use crate::errors::Error;
use crate::flatlander::Flatlander;
use std::io;


/// Lee una línea de stdin y devuelve un `Result`.
/// 
/// Devuelve: 
/// - Ok(`String`) contiene la línea leída.
/// - Err([`Error`]), de los cuales puede devolver: 
///   - [`Error::IO`]
///   - [`Error::LineaFaltante`]
fn leer_linea() -> Result<String, Error> {
    let mut linea: String = String::new();

    if let Err(_) = io::stdin().read_line(&mut linea) {
        return Err(Error::IO);
    }

    if linea.trim().is_empty() {
        return Err(Error::LineaFaltante);
    }

    Ok(linea)
}

/// Recibe un vector de strings [theta, n]. Los almacena, chequea que esten dentro de los rangos pedidos 
/// y los devuelve
/// 
/// Devuelve un `Result`: 
/// - Ok(`(f64, usize)`)
/// - Err([`Error`]), de los cuales puede ser:
///      - [`Error::ValorFaltante`]
///      - [`Error::FueraRango`]
///      - [`Error::NumeroInvalido`]
fn leer_theta_n(partes: Vec<&str>) -> Result<(f64, usize), Error> {
    if partes.len() < 2 {
        return Err(Error::ValorFaltante);
    }

    let theta: f64 = partes[0].parse().map_err(|_| Error::NumeroInvalido)?;

    if theta < 10. || theta > 80. {
        return Err(Error::FueraRango);
    };

    let n: usize = partes[1].parse().map_err(|_| Error::NumeroInvalido)?;

    if n <= 1 || n > usize::pow(10, 5) {
        return Err(Error::FueraRango);
    }

    Ok((theta, n))
}

/// Recibe un vector de strings [x, h]. Los almacena, chequea que esten dentro de los rangos pedidos 
/// y los devuelve
/// 
/// Devuelve un `Result`: 
/// - Ok([`crate::flatlander::Flatlander`])
/// - Err([`Error`]), de los cuales puede ser
///      - [`Error::ValorFaltante`]
///      - [`Error::FueraRango`]
///      - [`Error::NumeroInvalido`]
fn leer_flatlander(partes: Vec<&str>) -> Result<Flatlander, Error> {
    if partes.len() < 2 {
        return Err(Error::ValorFaltante);
    }

    let x: u32 = partes[0].parse().map_err(|_| Error::NumeroInvalido)?;

    if x > (3 * u32::pow(10, 5)) {
        // x < 0 esta puesto en NumeroInvalido, por problemas de parseo con el u32.
        return Err(Error::FueraRango);
    }

    let h: u32 = partes[1].parse().map_err(|_| Error::NumeroInvalido)?;

    if h < 1 || h > 1000 {
        return Err(Error::FueraRango);
    }

    Ok(Flatlander(x, h))
}

/// Se encarga de parsear todas las lineas recibidas por stdin, esta funcion utiliza las demas del módulo asi que puede devolver cualquier de los errores anteriormente descritos.
/// 
/// Devuelve un `Result`: 
/// - Ok((fa64, Vec<[`crate::flatlander::Flatlander`]>))
/// - Err([`Error`]).
pub fn parsear_lineas() -> Result<(f64, Vec<Flatlander>), Error> {
    let linea_header = leer_linea()?;

    let partes_header: Vec<&str> = linea_header.split_whitespace().collect();

    let (theta, n) = leer_theta_n(partes_header)?;

    let mut flatlanders: Vec<Flatlander> = Vec::new();
    for _ in 0..n {
        let linea_flatlander = leer_linea()?;

        let partes_flatlander: Vec<&str> = linea_flatlander.split_whitespace().collect();

        let f: Flatlander = leer_flatlander(partes_flatlander)?;
        flatlanders.push(f);
    }

    Ok((theta, flatlanders))
}
