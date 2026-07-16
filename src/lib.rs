//! Curso de sistemas operativos en Rust para Jeresoft Academy.
//!
//! Este crate acompaña el curso `rust-operating-systems`. Cada módulo futuro
//! representará un mecanismo canónico de sistemas operativos y existirá para
//! enseñar invariantes, límites, modos de falla y tradeoffs.
//!
//! La intención inicial no es construir un kernel de producción. La intención
//! es crear modelos educativos pequeños, verificables y bien documentados.

pub mod processes;

/// Devuelve el nombre canónico del curso.
///
/// # Examples
///
/// ```
/// assert_eq!(
///     rust_operating_systems::course_name(),
///     "rust-operating-systems"
/// );
/// ```
pub fn course_name() -> &'static str {
    "rust-operating-systems"
}
