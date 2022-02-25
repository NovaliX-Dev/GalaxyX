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

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

/// Draw a point in the canvas
pub fn draw_point(canvas: &mut Canvas<Window>, origin: (i32, i32), radius: u32, color: Color) {
    let rect = Rect::new(
        origin.0 - radius as i32,
        origin.1 - radius as i32, 
        radius * 2, 
        radius * 2
    );

    canvas.set_draw_color(color);
    canvas.fill_rect(rect).unwrap();
}
