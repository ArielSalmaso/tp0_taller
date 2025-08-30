/// Enum de errores utilizados a lo largo del proyecto
#[derive(Debug)] // esto es solamente para los tests de integracion
pub enum Error {
    /// Error en el Input/Ouput (IO)
    IO,
    /// Alguno de los valores superó las condiciones dadas
    FueraRango,
    /// Falta algún valor necesario
    ValorFaltante,
    /// Un numero no permitido a la hora de parsear el valor
    ///
    /// Ejemplo:
    /// ```rust
    ///     let s: &str = "-3";
    ///     let n: Result<u32, _> = s.parse();
    ///     assert!(n.is_err()); // NumeroInvalido
    /// ```
    NumeroInvalido,
    /// Falta de una linea al leer el stdin
    LineaFaltante,
}
