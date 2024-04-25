//! Fern reproduction.

use cells::{Cell, Gene};

/// A cell made by an adult fern. It disperses on the wind as part of
/// the fern life cycle. A spore grows into a prothallus -- a whole
/// separate organism, up to 5mm across -- which produces the zygote
/// that grows into a new fern. (Plant sex is complicated.)
pub struct Spore {
    pub size: f64
}

/// Simulate the production of a spore by meiosis.
pub fn produce_spore(_factory: &mut Sporangium) -> Spore {
    Spore { size: 1.0 }
}

#[allow(dead_code)]
/// Extract the genes in a particular spore.
pub(crate) fn genes(_spore: &Spore) -> Vec<Gene> {
    todo!()
}

#[allow(dead_code)]
/// Mix genes to prepare for meiosis (part of interphase).
fn recombine(_parent: &mut Cell) {
    todo!()
}

pub struct Sporangium;

mod cells {
    //! The simulation of biological cells, which is as low-level as we go.

    pub struct Cell {
        pub x: f64,
        pub y: f64
    }

    impl Cell {
        #[allow(dead_code)]
        pub fn distance_from_origin(&self) -> f64 {
            f64::hypot(self.x, self.y)
        }
    }

    pub struct Gene;
}