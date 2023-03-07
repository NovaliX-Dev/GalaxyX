use std::{f64::consts::PI, ops::Range};

use rand::{prelude::ThreadRng, Rng};
use sdl2::pixels::Color;

use crate::{common::vec2::Vec2F, object::Object, simulation::object::NegativeOrNullMassError};

/// Generate random points in a circle
fn generate_random_points_in_circle(
     rng: &mut ThreadRng,
     origin: Vec2F,
     r: f64,
     number: usize
) -> Vec<Vec2F> {
     let mut points = Vec::<Vec2F>::with_capacity(number);

     let mut i = 0;
     while i < number {
          let t = 2.0 * PI * rng.gen_range(0.0..r);
          let u = rng.gen_range(0.0..r) + rng.gen_range(0.0..r);
          let r = if u > r { 2.0 * r - u } else { u };

          points.push(Vec2F::new(r * t.cos(), r * t.sin()) + origin);

          i += 1;
     }

     points
}

/// Generate a random value in range
fn generate_random_values_in_range(
     rng: &mut ThreadRng,
     range: Range<f64>,
     number: usize
) -> Vec<f64> {
     let mut masses = Vec::<f64>::with_capacity(number);

     let mut i = 0;
     while i < number {
          masses.push(rng.gen_range(range.clone()));

          i += 1;
     }

     masses
}

/// Generate random object
pub fn generate_random_objects_in_circle(
     rng: &mut ThreadRng,
     origin: Vec2F,
     r: f64,
     mass_range: Range<f64>,
     number: usize,
     color: Color
) -> Result<Vec<Object>, NegativeOrNullMassError> {
     let points = generate_random_points_in_circle(rng, origin, r, number);

     let masses = generate_random_values_in_range(rng, mass_range, number);

     let mut objects = Vec::with_capacity(number);
     for (point, mass) in points.iter().zip(masses.iter()) {
          objects.push(Object::new_inactive(*mass, *point, true, color)?)
     }

     Ok(objects)
}
