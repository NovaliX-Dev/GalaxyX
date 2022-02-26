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

use super::Vec2F;

#[test]
fn test_from_angle_value() {
    assert_eq!(
        Vec2F::from_angle_value(0.0, 1.0),
        Vec2F::new(1.0, 0.0)
    );

    // others doesn't work if we compare the Vec2F itself,
    // because the supposed 0 is never 0.0, it's an approximation.
    assert_eq!(
        Vec2F::from_angle_value(PI, 1.0).x,
        -1.0
    );
    assert_eq!(
        Vec2F::from_angle_value(FRAC_PI_2, 1.0).y,
        1.0
    );
    assert_eq!(
        Vec2F::from_angle_value(-FRAC_PI_2, 1.0).y,
        -1.0
    );
}
