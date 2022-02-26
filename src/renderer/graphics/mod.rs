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

mod draw;
pub mod object_radius;

use sdl2::{render::Canvas, video::Window};

use crate::simulation::object::Object;

use self::object_radius::RadiusType;

// =============================================================================
// Type
// =============================================================================

pub struct Graphics {
    radius_type: RadiusType,
}

impl Graphics {
    pub fn new(radius_type: RadiusType) -> Self { 
        Self { radius_type } 
    }
}

// =============================================================================
// Functions
// =============================================================================

/// Draw an object in the canvas
pub fn draw_object(canvas: &mut Canvas<Window>, object: &Object, settings: &Graphics) {
    let p = object.location;
    let x = p.x as i32;
    let y = p.y as i32;

    // compute the radius
    let r = match &settings.radius_type {
        RadiusType::Constant(v) => *v,
        RadiusType::FromMass(v) => v.radius_u32_from_mass(object.mass),
    };

    draw::draw_point_u32(canvas, (x, y), r, object.color);
}
