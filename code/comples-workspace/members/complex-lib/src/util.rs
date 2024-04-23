//! Complex number examples.
//!
//! The chapter presents several different variations on how one might define
//! arithmetic on a generic `Complex` type, so what we have here are a bunch of
//! isolated modules, each of which defines its own `Complex` type in its own
//! way. The `first_cut` module is the most well-developed.
//!
//! If you actually need a `Complex` type for real use, consider the
//! `num_complex` crate, whose `Complex` type is incorporated into the `num`
//! crate.

#[macro_export]
macro_rules! define_complex {
  () => {
      #[derive(Clone, Copy, Debug)]
      struct Complex<T> {
          /// Real portion of the complex number
          re: T,

          /// Imaginary portion of the complex number
          im: T,
      }
  };
}