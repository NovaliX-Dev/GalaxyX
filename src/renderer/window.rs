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

use sdl2::{render::Canvas, video::Window, VideoSubsystem};

/// Create a window with it's canvas
pub fn create(
     video: VideoSubsystem,
     width: u32,
     height: u32,
     title: &str,
     resizable: bool
) -> Result<Canvas<Window>, String> {
     // create and configure the window builder
     let mut window_builder = video.window(title, width, height);
     if resizable {
          window_builder.resizable();
     }

     // build the window
     let window = window_builder.build().map_err(|e| e.to_string())?;

     // create the canvas from the window
     let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

     Ok(canvas)
}
