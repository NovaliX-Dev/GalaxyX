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
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;

use crate::common::vec2::Vec2;
use crate::renderer::graphics::{self, Graphics};

use crate::renderer::viewport::Viewport;
use crate::simulation::thread;
use crate::{
     renderer,
     simulation::{object::Object, physics},
};

/// Create all the threads for the simulation and launch the window
pub fn run(
     mut objects: Vec<Object>,
     delta_t: f64,
     force_smoothings: f64,
     graphics: Graphics,
     mut viewport: Viewport,
     window_size: Vec2<u32>,
) -> anyhow::Result<()> {
     // -------------------------------------------------------------------------
     // Window creation
     // -------------------------------------------------------------------------

     // init sdl modules
     let (video, mut event_pump, mouse) = renderer::init_sdl_modules()
          .map_err(|e| anyhow::anyhow!(e))
          .with_context(|| "Couldn't initialize SDL modules.")?;

     // create the window
     let mut canvas =
          renderer::window::create(video, window_size.x, window_size.y, "GalaxyX", true)
               .map_err(|e| anyhow::anyhow!(e))
               .with_context(|| "Couldn't create the window.")?;

     // ------------------------------------------------------------------------
     // Thread launch
     // ------------------------------------------------------------------------

     let mut objects_to_draw = objects.clone();
     let receiver = thread::launch_engine_thread(objects, force_smoothings, delta_t);

     // -------------------------------------------------------------------------
     // Window loop
     // -------------------------------------------------------------------------

     let mut time_passed = 0.0;

     'win_loop: loop {
          // handle events if any
          for event in event_pump.poll_iter() {
               match event {
                    // window close, since there is only one
                    Event::Quit { .. } => break 'win_loop,

                    // -------------------------------------------------------------
                    // Viewport controls
                    // -------------------------------------------------------------
                    Event::MouseWheel { y, .. } => {
                         let center = window_size / 2;

                         viewport.zoom(y as f64, center.convert_as_to_type());
                    },

                    Event::MouseMotion {
                         xrel,
                         yrel,
                         mousestate,
                         ..
                    } => {
                         if mousestate.left() {
                              mouse.set_relative_mouse_mode(true);

                              viewport.move_(xrel, yrel);
                         }
                    },

                    Event::MouseButtonUp { mouse_btn, .. } => {
                         if mouse_btn == MouseButton::Left {
                              mouse.set_relative_mouse_mode(false);
                         }
                    },

                    _ => (),
               }
          }

          // get a first time value
          let time_now = Instant::now();

          // ---------------------------------------------------------------------
          // Physics computation
          // ---------------------------------------------------------------------

          if let Ok(new_data) = receiver.try_recv() {
               objects_to_draw = new_data;
          }

          // ---------------------------------------------------------------------
          // Rendering
          // ---------------------------------------------------------------------

          canvas.set_draw_color(Color::BLACK);
          canvas.clear();

          for o in objects_to_draw.iter() {
               graphics::draw_object(&mut canvas, o, &graphics, &viewport)
          }

          canvas.present();

          // compute the time passed during the physics computation and display
          time_passed = time_now.elapsed().as_secs_f64();
     }

     Ok(())
}
