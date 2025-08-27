use crate::errors::Error;
use crate::flatlander::Flatlander;
use std::io;
use std::iter::FusedIterator;

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

fn leer_flatlander(partes: Vec<&str>) -> Result<Flatlander, Error> {
    if partes.len() < 2 {
        return Err(Error::ValorFaltante);
    }

    //(0 <= X <= 3 . 10^5) ^ (1 <= H <= 1000)
    let x: u32 = partes[0].parse().map_err(|_| Error::NumeroInvalido)?;

    if x > (3 * u32::pow(10, 5)) { // x < 0 esta puesto en NumeroInvalido, por problemas de parseo con el u32.
        return Err(Error::FueraRango);
    }

    let h: u32 = partes[1].parse().map_err(|_| Error::NumeroInvalido)?;

    if h < 1 || h > 1000 {
        return Err(Error::FueraRango);
    }

    Ok(Flatlander(x, h))
}

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

    Ok((theta,flatlanders))
}
