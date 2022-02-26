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

// =============================================================================
// Enum
// =============================================================================

pub enum RadiusType {
    Constant(u32),
    FromMass(MassGraphics)
}

// =============================================================================
// Types
// =============================================================================

/// allow to compute the radius of an object from it's mass
pub struct MassGraphics {
    min_size: u32,
    mass_factor: f64
}

impl MassGraphics {
    pub fn new(min_size: u32, mass_factor: f64) -> Self { 
        Self { min_size, mass_factor } 
    }
 
    /// compute the radius of an object from it's mass
    pub fn radius_u32_from_mass(&self, mass: f64) -> u32 {
        let a = mass * self.mass_factor;

        a as u32 + self.min_size
    }
}
