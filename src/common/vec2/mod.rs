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

use std::fmt::Debug;
use std::ops::{Div, Mul, AddAssign, Sub};

use num_traits::{Float, Num};

// =============================================================================
// Type
// =============================================================================

/// A group of two values
#[derive(Clone, Copy)]
pub struct Vec2<T> where T: Num {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> where T: Num {
    /// Construct a new Vec2F object
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// Construct a new null Vec2F object
    pub fn new_null() -> Self {
        Self { x: T::zero(), y: T::zero() }
    }
}

impl<T> Vec2<T> where T: Float {
    /// Create a vector from a value and angle
    pub fn from_angle_value(a: T, v: T) -> Self {
        Self {
            x: a.cos() * v,
            y: a.sin() * v
        }
    }
}

impl<T> Sub for Vec2<T> where T: Num {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl<T> Div<T> for Vec2<T> where T: Num + Copy {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs
        }
    }
}

impl<T> Mul<T> for Vec2<T> where T: Num + Copy {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl<T> AddAssign for Vec2<T> where T: Num + AddAssign {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> PartialEq for Vec2<T> where T: Num {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T> Debug for Vec2<T> where T: Num + Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vec2F").field("x", &self.x).field("y", &self.y).finish()
    }
}

// =============================================================================
// Alias
// =============================================================================

pub type Vec2F = Vec2<f64>;
