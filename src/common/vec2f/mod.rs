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

use std::fmt::Debug;
use std::ops::{Div, Mul, AddAssign, Sub};

/// A group of two f64 values
#[derive(Clone, Copy)]
pub struct Vec2F {
    pub x: f64,
    pub y: f64,
}

impl Vec2F {
    /// Construct a new Vec2F object
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Construct a new null Vec2F object
    pub fn new_null() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// Create a vector from a value and angle
    pub fn from_angle_value(a: f64, v: f64) -> Self {
        Self {
            x: a.cos() * v,
            y: a.sin() * v
        }
    }
}

impl Sub for Vec2F {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl Div<f64> for Vec2F {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs
        }
    }
}

impl Mul<f64> for Vec2F {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl AddAssign for Vec2F {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl PartialEq for Vec2F {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Debug for Vec2F {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vec2F").field("x", &self.x).field("y", &self.y).finish()
    }
}
