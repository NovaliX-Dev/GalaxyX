// A galaxy simulator made in Rust.
// Copyright (C) 2022 NovaliX
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

mod initial;
pub(crate) mod macros;
pub use initial::*;

use std::fmt::{Debug, Display};

use sdl2::pixels::Color;

use crate::common::vec2::Vec2F;

// =============================================================================
// Error
// =============================================================================

pub struct NegativeOrNullMassError {
     mass: f64,
}

impl Debug for NegativeOrNullMassError {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
          f.debug_struct("NegativeOrNullMassError")
               .field("mass", &self.mass)
               .finish()
     }
}

impl Display for NegativeOrNullMassError {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
          if self.mass == 0.0 {
               f.write_str("The mass of the object if null.")
          } else {
               f.write_str(
                    format!("The mass of the object is negative ({} < 0)", self.mass).as_str(),
               )
          }
     }
}

// =============================================================================
// Type
// =============================================================================

/// The type used by the physics engine
#[derive(Clone, Copy)]
pub struct Object {
     pub mass: f64,
     pub location: Vec2F,
     pub force: Vec2F,
     pub velocity: Vec2F,
     pub can_move: bool,
     pub color: Color,
}

impl Object {
     /// Create a new object if the mass is valid.
     /// Else return an error.
     pub fn new(
          mass: f64,
          location: Vec2F,
          force: Vec2F,
          velocity: Vec2F,
          can_move: bool,
          color: Color,
     ) -> Result<Self, NegativeOrNullMassError> {
          if mass <= 0.0 {
               Err(NegativeOrNullMassError { mass })
          } else {
               Ok(Self {
                    mass,
                    location,
                    force,
                    velocity,
                    can_move,
                    color,
               })
          }
     }

     /// Create a new inactive object if the mass is valid.
     /// Else return an error.
     pub fn new_inactive(
          mass: f64,
          location: Vec2F,
          can_move: bool,
          color: Color,
     ) -> Result<Self, NegativeOrNullMassError> {
          Self::new(
               mass,
               location,
               Vec2F::new_null(),
               Vec2F::new_null(),
               can_move,
               color,
          )
     }
}
