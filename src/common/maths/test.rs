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

use std::f64::consts::{PI, FRAC_PI_2};

use super::{compute_angle, compute_distance};
use crate::common::vec2f::Vec2F;

#[test]
fn test_compute_angle() {
    assert_eq!(
        compute_angle(Vec2F::new(0.0, 0.0), Vec2F::new(1.0, 0.0)),
        0.0
    );
    assert_eq!(
        compute_angle(Vec2F::new(0.0, 0.0), Vec2F::new(0.0, 1.0)),
        FRAC_PI_2
    );
    assert_eq!(
        compute_angle(Vec2F::new(0.0, 0.0), Vec2F::new(-1.0, 0.0)),
        PI
    );
    assert_eq!(
        compute_angle(Vec2F::new(0.0, 0.0), Vec2F::new(0.0, -1.0)),
        -FRAC_PI_2
    );
}

#[test]
fn test_compute_distance() {
    assert_eq!(
        compute_distance(Vec2F::new(0.0, 0.0), Vec2F::new(1.0, 0.0)),
        1.0
    );
    assert_eq!(
        compute_distance(Vec2F::new(0.0, 0.0), Vec2F::new(0.0, 1.0)),
        1.0
    );
    assert_eq!(
        compute_distance(Vec2F::new(0.0, 0.0), Vec2F::new(1.0, 1.0)),
        2.0_f64.sqrt()
    );
}
