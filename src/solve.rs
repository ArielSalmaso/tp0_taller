use crate::flatlander::Flatlander;
/// Este struct almacena los datos del problema planteado para poder resolverlo
pub struct Solver {
    /// Un vector de flatlanders, veasé [`crate::flatlander::Flatlander`].
    flatlanders: Vec<Flatlander>,
    /// La tangente del angulo recibido.
    tan_theta: f64,
}

/// Función que calcula la tangente del angulo recibido por parametro
fn calcular_tan(theta: f64) -> f64 {
    let theta_rads = theta.to_radians();
    let tan_theta = theta_rads.tan();

    tan_theta
}

impl Solver {
    /// Constructor del struct Solver
    pub fn new(flatlanders: Vec<Flatlander>, theta: f64) -> Self {
        let tan_theta: f64 = calcular_tan(theta);
        Solver {
            flatlanders,
            tan_theta,
        }
    }

    /// Método que devuelve el calulo de las sombras de los flatlanders (atributo del solver).
    /// Para resolverlo primero se ordena el array de flatanders segun la coordenada x, y luego se hace el calculo de cada uno
    /// teniendo en cuenta si se superponen o no.
    /// Complejidad del algoritmo: O(n log n), siendo n la cantidad de flatlanders.
    pub fn solve(&mut self) -> f64 {
        if self.flatlanders.len() == 0 {
            return 0.0; //este caso nunca deberia pasar, por el tema de los rangos pedidos del tp.
        }
        self.flatlanders.sort_by_key(|f| f.0);
        let mut total: f64 = 0.;
        let mut inicio = self.flatlanders[0].0 as f64;
        let mut fin = inicio + (self.flatlanders[0].1 as f64 / self.tan_theta);

        for flatlander in &self.flatlanders[1..] {
            let h = flatlander.1 as f64;
            let l = h / self.tan_theta;
            let inicio_actual = flatlander.0 as f64;
            let fin_actual = inicio_actual + l;

            //el flatlander se superpone
            if inicio_actual < fin {
                if fin_actual > fin {
                    fin = fin_actual;
                }
            } else {
                total += fin - inicio;

                inicio = inicio_actual;
                fin = fin_actual;
            }
        }

        total += fin - inicio;
        return total;
    }
}

#[cfg(test)]
mod tests {
    use super::*; //importa todas las funciones del módulo padre, en este caso solve.

    fn approx_eq(a: f64, b: f64, eps: f64) -> bool {
        (a - b).abs() < eps
    }

    #[test]
    fn test_enunciado() {
        let mut solver = Solver::new(
            vec![
                Flatlander(50, 150),
                Flatlander(0, 100),
                Flatlander(100, 200),
            ],
            30.0,
        );

        let res = solver.solve();
        assert!(approx_eq(res, 446.4101615137755, 1e-9));
    }

    #[test]
    fn test_enunciado2() {
        let mut solver = Solver::new(
            vec![
                Flatlander(50, 150),
                Flatlander(0, 100),
                Flatlander(100, 200),
            ],
            45.0,
        );

        let res = solver.solve();
        assert!(approx_eq(res, 300.00000000000006, 1e-9));
    }

    #[test]
    fn test_un_solo_flatlander() {
        let mut solver = Solver::new(vec![Flatlander(10, 100)], 45.0);

        let res = solver.solve();
        assert!(approx_eq(res, 100.0, 1e-9));
    }

    #[test]
    fn test_ninguno_se_superpone() {
        let mut solver = Solver::new(vec![Flatlander(0, 100), Flatlander(200, 100)], 45.0);

        let res = solver.solve();
        assert!(approx_eq(res, 200.0, 1e-9));
    }

    #[test]
    fn test_todos_se_superponen() {
        let mut solver = Solver::new(vec![Flatlander(0, 100), Flatlander(0, 200)], 45.0);

        let res = solver.solve();
        assert!(approx_eq(res, 200.0, 1e-9));
    }
}
