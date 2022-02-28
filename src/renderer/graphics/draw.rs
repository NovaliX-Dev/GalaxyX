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
use sdl2::rect::{Point, Rect};
use sdl2::render::{BlendMode, Canvas};
use sdl2::video::Window;

use crate::common::vec2::Vec2;

// =============================================================================
// Point
// =============================================================================

/// Draw a point in the canvas
pub fn draw_point_u32(canvas: &mut Canvas<Window>, origin: Vec2<i32>, radius: u32, color: Color) {
    let rect = Rect::new(
        origin.x - radius as i32,
        origin.y - radius as i32,
        radius * 2,
        radius * 2,
    );

    canvas.set_draw_color(color);
    canvas.fill_rect(rect).unwrap();
}

/// Draw a point in the canvas with radius anti-aliasing
pub fn draw_point_u32_radius_f64(
    canvas: &mut Canvas<Window>,
    origin: Vec2<i32>,
    radius: f64,
    color: Color,
) {
    let r_max = radius.ceil();
    let r_min = radius.floor();

    // if the radius is an integer we can draw it only once
    if radius == r_max {
        draw_point_u32(canvas, origin, radius as u32, color);

        return;
    }

    // compute color from ceil and floor
    let r_max_alpha = color.a as f64 * (r_max - radius);
    let r_min_alpha = color.a as f64 * (radius - r_min);

    let r_max_color = Color::RGBA(color.r, color.g, color.b, r_max_alpha as u8);
    let r_min_color = Color::RGBA(color.r, color.g, color.b, r_min_alpha as u8);

    draw_point_u32(canvas, origin, r_max as u32, r_max_color);
    draw_point_u32(canvas, origin, r_min as u32, r_min_color);
}

/// Draw a point in the canvas with radius and location anti-aliasing
pub fn draw_point_f64_radius_f64(
    canvas: &mut Canvas<Window>,
    origin: Vec2<f64>,
    radius: f64,
    color: Color,
) {
    // get ceil and floor x and y
    let (max_x_i32, max_y_i32) = origin.convert(|v| v.ceil()).as_tuple();
    let (min_x_i32, min_y_i32) = origin.convert(|v| v.floor()).as_tuple();

    // if the origin is on a pixel directly we have to draw it only one time
    if (max_x_i32, max_y_i32) == origin.as_tuple() {
        draw_point_u32_radius_f64(canvas, origin.convert(|v| v as i32), radius, color);

        return;
    }

    // compute alpha values
    let max_y_alpha_f = max_y_i32 - origin.y;
    let min_y_alpha_f = origin.y - min_y_i32;
    let max_x_alpha_f = max_x_i32 - origin.x;
    let min_x_alpha_f = origin.x - min_x_i32;

    let max_y_min_x_alpha_f = max_y_alpha_f * min_x_alpha_f;
    let max_y_max_x_alpha_f = max_y_alpha_f * max_x_alpha_f;
    let min_y_min_x_alpha_f = min_y_alpha_f * min_x_alpha_f;
    let min_y_max_x_alpha_f = min_y_alpha_f * max_x_alpha_f;

    let max_y_min_x_alpha = max_y_min_x_alpha_f * color.a as f64;
    let max_y_max_x_alpha = max_y_max_x_alpha_f * color.a as f64;
    let min_y_min_x_alpha = min_y_min_x_alpha_f * color.a as f64;
    let min_y_max_x_alpha = min_y_max_x_alpha_f * color.a as f64;

    // compute colors
    let max_y_min_x_color = Color::RGBA(color.r, color.g, color.b, max_y_min_x_alpha.round() as u8);
    let max_y_max_x_color = Color::RGBA(color.r, color.g, color.b, max_y_max_x_alpha.round() as u8);
    let min_y_min_x_color = Color::RGBA(color.r, color.g, color.b, min_y_min_x_alpha.round() as u8);
    let min_y_max_x_color = Color::RGBA(color.r, color.g, color.b, min_y_max_x_alpha.round() as u8);

    // compute origins
    let min_y_max_x_origin = Vec2::new(min_x_i32 as i32, max_y_i32 as i32);
    let min_y_min_x_origin = Vec2::new(max_x_i32 as i32, max_y_i32 as i32);
    let max_y_max_x_origin = Vec2::new(min_x_i32 as i32, min_y_i32 as i32);
    let max_y_min_x_origin = Vec2::new(max_x_i32 as i32, min_y_i32 as i32);

    // draw colors
    canvas.set_blend_mode(BlendMode::Add);
    draw_point_u32_radius_f64(canvas, min_y_max_x_origin, radius, min_y_max_x_color);
    draw_point_u32_radius_f64(canvas, min_y_min_x_origin, radius, min_y_min_x_color);
    draw_point_u32_radius_f64(canvas, max_y_max_x_origin, radius, max_y_max_x_color);
    draw_point_u32_radius_f64(canvas, max_y_min_x_origin, radius, max_y_min_x_color);
}

// =============================================================================
// Line
// =============================================================================

/// Draw a vector in the canvas
pub fn draw_line_u32(canvas: &mut Canvas<Window>, p1: Vec2<i32>, p2: Vec2<i32>, color: Color) {
    // skip empty lines
    if p1 == p2 {
        return;
    }

    canvas.set_draw_color(color);

    // convert Vec2 to SDL points
    let point1 = Point::new(p1.x, p1.y);
    let point2 = Point::new(p2.x, p2.y);

    canvas.draw_line(point1, point2).unwrap();
}
