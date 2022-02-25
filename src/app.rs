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

use std::time::Instant;

use anyhow::Context;

use sdl2::event::Event;
use sdl2::pixels::Color;

use crate::renderer::graphics;
use crate::{simulation::{object::Object, physics}, renderer};

/// Create all the threads for the simulation and launch the window
pub fn run(
    mut objects: Vec<Object>,
    delta_t: f64
) -> anyhow::Result<()> {
    // -------------------------------------------------------------------------
    // Window creation
    // -------------------------------------------------------------------------

    // init sdl modules
    let (video, mut event_pump) = renderer::init_sdl_modules()
        .map_err(|e| anyhow::anyhow!(e))
        .with_context(|| "Couldn't initialize SDL modules.")?;

    // create the window
    let mut canvas = renderer::window::create(
        video, 
        800, 
        600, 
        "GalaxyX", 
        true
    )
        .map_err(|e| anyhow::anyhow!(e))
        .with_context(|| "Couldn't create the window.")?;

    // -------------------------------------------------------------------------
    // Window loop
    // -------------------------------------------------------------------------

    'win_loop: loop {
        // handle events if any
        for event in event_pump.poll_iter() {
            match event {
                // window close, since there is only one
                Event::Quit {..} => break 'win_loop,

                _ => ()
            }
        }

        // ---------------------------------------------------------------------
        // Physics computation
        // ---------------------------------------------------------------------

        physics::compute_object_global_force_for_each(&mut objects);
        physics::compute_object_next_position_for_each(&mut objects, delta_t);

        // ---------------------------------------------------------------------
        // Rendering
        // ---------------------------------------------------------------------

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        for o in objects.iter() {
            let p = o.location;
            let x = p.x as i32;
            let y = p.y as i32;

            graphics::draw_point(&mut canvas, (x, y), 3, o.color);
        }

        canvas.present();
    }

    Ok(())
}
