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

use crate::{
     common::vec2::{Vec2, Vec2F},
     object::Object,
};

const MIN_ZOOM: f64 = 10e-3;

pub struct Viewport {
     original_scale: f64,
     pub scale: f64,
     pub shift: Vec2F,
     absolute_shift: Vec2F,
}

impl Viewport {
     pub fn new(scale: f64, shift: Vec2F) -> Self {
          Self {
               scale,
               shift,
               original_scale: scale,
               absolute_shift: shift,
          }
     }

     pub fn move_(&mut self, x: i32, y: i32) {
          self.move_f64(Vec2F::new(x.into(), y.into()))
     }

     fn move_f64(&mut self, move_: Vec2F) {
          self.shift += move_;

          self.absolute_shift += move_ / self.scale;
     }

     pub fn zoom(&mut self, delta_wheel: f64, delta_center: Vec2F) {
          let delta_scale = delta_wheel * 0.05 * self.original_scale;
          self.scale += delta_scale;

          if self.scale < MIN_ZOOM {
               self.scale = MIN_ZOOM;

               return;
          }

          self.shift -= (delta_center - self.absolute_shift) * delta_scale;
     }
}
