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

pub mod graphics;
pub mod viewport;
pub mod window;

use sdl2::{mouse::MouseUtil, EventPump, VideoSubsystem};

/// Initialize all needed SDL modules
pub fn init_sdl_modules() -> Result<(VideoSubsystem, EventPump, MouseUtil), String> {
    let sdl_context = sdl2::init()?;

    let video = sdl_context.video()?;
    let event_pump = sdl_context.event_pump()?;

    let mouse = sdl_context.mouse();

    Ok((video, event_pump, mouse))
}
