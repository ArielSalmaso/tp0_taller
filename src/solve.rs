use crate::flatlander::{self, Flatlander};
use crate::errors::Errors;

pub struct Solver {
    flatlanders: Vec<Flatlander>,
    tan_theta: f64,
}

fn calcular_tan(theta: f64) -> Result<f64, Errors> {
    /*
    Calcula la tangente del angulo pasado, primero pasa el angulo a radianes y luego realiza el calculo.
    Recibe: 
        Un angulo en grados en el rango de [10,80], sino devuelve un FueraRango -> realmente los chequeos se hacen antes
    Devuelve:
        La tangente del angulo 
     */
    if theta < 10. || theta > 80. {
        return Err(Errors::FueraRango);
    }
    let theta_rads = theta.to_radians();
    let tan_theta = theta_rads.tan();

    Ok(tan_theta)
}

impl Solver {
    pub fn new(flatlanders: Vec<Flatlander>, theta: f64) -> Result<Self, Errors> {
        /*
        Constructor del struct solver.
        Recibe:
            Un vector de flatlanders y un angulo.
        Devuelve:
            Un struct solver.
         */
        let tan_theta: f64 = calcular_tan(theta)?;
        Ok(Solver {
            flatlanders,
            tan_theta,
        })
    }

    pub fn solve(&mut self) -> f64 {
        //Se ordena de manera ascendente
        self.flatlanders.sort_by_key(|f| f.0); 
        let mut total:f64 = 0.; 
        let mut inicio = self.flatlanders[0].0 as f64;
        let mut fin = inicio + (self.flatlanders[0].1 as f64 / self.tan_theta);


        for flatlander in &self.flatlanders[1..]{
            let h = flatlander.1 as f64;
            let l = h  / self.tan_theta;
            let inicio_actual = flatlander.0 as f64;
            let fin_actual = inicio_actual + l;


            if inicio_actual < fin{

                if fin_actual > fin{ 
                    fin = fin_actual;
                }
            } 
            else{
                total += fin - inicio;
                 
                inicio  = inicio_actual;
                fin = fin_actual;
            }

        }

        total += fin - inicio;
        return total;
    }
}



//tests