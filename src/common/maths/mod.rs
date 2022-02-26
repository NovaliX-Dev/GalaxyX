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

#[cfg(test)]
mod test;

use super::vec2::Vec2F;

/// Compute the distance between p1 and p2
pub fn compute_distance(p1: Vec2F, p2: Vec2F) -> f64 {
    ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt()
}

/// Compute the angle between origin and a
pub fn compute_angle(origin: Vec2F, a: Vec2F) -> f64 {
    let vec = a - origin;
    let ref_vec = Vec2F::new(1.0, 0.0); // the vec for which the angle is 0

    vec.y.atan2(vec.x) - ref_vec.y.atan2(ref_vec.x)
}