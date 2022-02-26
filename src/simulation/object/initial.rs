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

pub enum VelocityDirection {
    Left,
    Right,
}

// =============================================================================
// Function
// =============================================================================

use std::f64::consts::FRAC_PI_2;

use crate::common::{constants::G, maths, vec2::Vec2F};

use super::Object;

/// Make the object o turn around many others objects (called origins)
pub fn add_orbital_velocity(o: &mut Object, origins: &Vec<Object>, direction: VelocityDirection) {
    for origin in origins {
        let d = maths::compute_distance(o.location, origin.location);

        if d == 0.0 {
            continue;
        }

        let v = ((G * origin.mass) / d).sqrt();

        let a = maths::compute_angle(o.location, origin.location);
        let v_vec = Vec2F::from_angle_value(
            match direction {
                VelocityDirection::Left => a + FRAC_PI_2,
                VelocityDirection::Right => a - FRAC_PI_2,
            },
            v,
        );

        o.velocity += v_vec;
    }
}
