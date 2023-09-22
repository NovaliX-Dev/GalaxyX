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

use anyhow::Context;
use common::vec2::{Vec2, Vec2F};
use rand::thread_rng;
use renderer::{
     graphics::{
          radius::{MassGraphics, RadiusType},
          vectors::{ForceLengthType, VelocityLengthType},
          Graphics
     },
     viewport
};
use sdl2::pixels::Color;
use simulation::object;

use crate::simulation::object::{macros::create_object_value_checked, Object};

mod app;
mod common;
mod generation;
mod renderer;
mod simulation;

fn main() -> anyhow::Result<()> {
     let mut rng = thread_rng();

     // -------------------------------------------------------------------------
     // Objects creation and configuration
     // -------------------------------------------------------------------------

     let bh1 = create_object_value_checked!(1000.0, Vec2F::new(0.0, 0.0), false, Color::RED);

     // let mut s1 = create_object_value_checked!(50.0, Vec2F::new(400.0, 200.0),
     // true, Color::CYAN);
     let mut objects = generation::generate_random_objects_in_circle(
          &mut rng,
          bh1.location,
          1000.0,
          10.0..100.0,
          500,
          Color::CYAN
     )
     .map_err(|e| anyhow::anyhow!(e))?;

     object::add_orbital_velocity_for_each(
          &mut objects,
          &vec![bh1],
          object::VelocityDirection::Left
     );

     objects.push(bh1);

     // -------------------------------------------------------------------------
     // Graphics configuration
     // -------------------------------------------------------------------------

     let mass_graphics = MassGraphics::new(5.5, 0.0222);
     let radius_type = RadiusType::FromMass(mass_graphics);

     let graphics = Graphics::new(
          radius_type,
          Some(VelocityLengthType::Constant(20.0)),
          Some(ForceLengthType::Constant(10.0))
     );

     let window_size = Vec2::new(1024, 768);
     let viewport = viewport::Viewport::new(1.0, Vec2F::new_null());

     // launch the app
     app::run(objects, 36000.0, 50.0, 5, graphics, viewport, window_size)
}
