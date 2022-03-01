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

use std::mem::swap;

use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};

use crate::common::vec2::{Vec2, Vec2F};

/// Draw a vector in the canvas
pub fn draw_line_u32(canvas: &mut Canvas<Window>, p1: Vec2<i32>, p2: Vec2<i32>, color: Color) {
     // skip empty lines
     if p1 == p2 {
          return;
     }

     if canvas.draw_color() != color {
          canvas.set_draw_color(color);
     }

     // convert Vec2 to SDL points
     let point1 = Point::new(p1.x, p1.y);
     let point2 = Point::new(p2.x, p2.y);

     canvas.draw_line(point1, point2).unwrap();
}
