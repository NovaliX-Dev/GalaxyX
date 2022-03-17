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

use super::object::Object;
use crate::common::constants::G;
use crate::common::maths::{self, compute_angle};
use crate::common::vec2::Vec2F;

fn compute_force_value(o1: &Object, o2: &Object, force_smoothings: f64) -> f64 {
     let d = maths::compute_distance(o1.location, o2.location);
     if d == 0.0 {
          0.0
     } else {
          G * (o1.mass * o2.mass) / (d.powi(2) + force_smoothings)
     }
}

/// Compute the global force the object is affected by
pub fn compute_object_global_force(
     object: &mut Object,
     others: &Vec<Object>,
     force_smoothings: f64,
) {
     let mut global_f_vec = Vec2F::new_null();
     for o2 in others {
          let f = compute_force_value(object, o2, force_smoothings);
          let a = compute_angle(object.location, o2.location);

          let f_vec = Vec2F::from_angle_value(a, f);
          global_f_vec += f_vec;
     }

     object.force = global_f_vec;
}

/// Compute the global force each object is affected by
pub fn compute_object_global_force_for_each(objects: &mut Vec<Object>, force_smoothings: f64) {
     for i in 0..objects.len() {
          if !objects.get(i).unwrap().can_move {
               continue;
          }

          let mut o_vec2 = objects.clone();
          o_vec2.remove(i);

          compute_object_global_force(objects.get_mut(i).unwrap(), &o_vec2, force_smoothings)
     }
}

/// Compute the object's next position
pub fn compute_object_next_position(object: &mut Object, delta_t: f64) {
     let a = object.force / object.mass;
     object.velocity += a * delta_t;

     object.location += object.velocity * delta_t;
}

/// Compute all objects' next position
pub fn compute_object_next_position_for_each(objects: &mut Vec<Object>, delta_t: f64) {
     for o in objects {
          if !o.can_move {
               continue;
          }

          compute_object_next_position(o, delta_t)
     }
}
