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
pub mod radius;
pub mod vectors;

use sdl2::{pixels::Color, render::Canvas, video::Window};

use self::{
     radius::RadiusType,
     vectors::{ForceLengthType, VelocityLengthType}
};
use super::viewport::Viewport;
use crate::{
     common::{
          maths,
          vec2::{Vec2F, VecLength}
     },
     simulation::object::Object
};

// =============================================================================
// Type
// =============================================================================

pub struct Graphics {
     radius_type: RadiusType,
     velocity: Option<VelocityLengthType>,
     force: Option<ForceLengthType>
}

impl Graphics {
     pub fn new(
          radius_type: RadiusType,
          velocity: Option<VelocityLengthType>,
          force: Option<ForceLengthType>
     ) -> Self {
          Self {
               radius_type,
               velocity,
               force
          }
     }
}

// =============================================================================
// Functions
// =============================================================================

macro_rules! draw_vector_option {
     (
          $canvas: ident,
          $settings: ident,
          $attribute: ident,
          $factor: expr,
          $enum: ident,
          $object: ident,
          $object_attribute: ident,
          $object_origin: ident,
          $color: expr,
          $viewport: ident
    ) => {
          if let Some(v) = &$settings.$attribute {
               if $object.$object_attribute.length_f64() != 0.0 {
                    let p2 = match v {
                         $enum::Constant(v) => Vec2F::from_angle_value(
                              maths::compute_angle(Vec2F::new_null(), $object.$object_attribute),
                              *v
                         ),
                         $enum::FromValueFactor(f) => $object.$object_attribute * *f
                    };

                    let f_vector = $object.location + p2;
                    let f_vector_scaled = f_vector * $viewport.scale + $viewport.shift;

                    // draw it's force
                    draw::draw_line_u32(
                         $canvas,
                         $object_origin,
                         f_vector_scaled.convert(|v| v as i32),
                         $color
                    );
               }
          }
     };
}

/// Draw an object in the canvas
pub fn draw_object(
     canvas: &mut Canvas<Window>,
     object: &Object,
     settings: &Graphics,
     viewport: &Viewport
) {
     let location = object.location * viewport.scale + viewport.shift;
     let p = location.convert(|v| v as i32);

     // compute the radius
     let r = match &settings.radius_type {
          RadiusType::Constant(v) => *v,
          RadiusType::FromMass(v) => v.radius_f64_from_mass(object.mass)
     };
     let radius = viewport.scale * r;

     // draw the object
     draw::draw_point_f64_radius_f64(canvas, location, radius, object.color);

     // draw it's force if requested
     draw_vector_option!(
          canvas,
          settings,
          force,
          10.0e8,
          ForceLengthType,
          object,
          force,
          p,
          Color::WHITE,
          viewport
     );

     // draw it's velocity if requested
     draw_vector_option!(
          canvas,
          settings,
          velocity,
          10.0e5,
          VelocityLengthType,
          object,
          velocity,
          p,
          Color::WHITE,
          viewport
     );
}
