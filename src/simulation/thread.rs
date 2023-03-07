use std::{
     sync::mpsc::{self, Receiver, Sender},
     thread,
};

use super::physics;
use crate::object::Object;

/// Function in the engine thread
fn engine_thread(
     mut objects: Vec<Object>,
     sender: Sender<Vec<Object>>,
     force_smoothings: f64,
     delta_t: f64,
) {
     loop {
          physics::compute_object_global_force_for_each(&mut objects, force_smoothings);
          physics::compute_object_next_position_for_each(&mut objects, delta_t);

          let r = sender.send(objects.clone());
          if let Err(_) = r {
               break;
          }
     }
}

/// Init the engine thread
pub fn launch_engine_thread(
     objects: Vec<Object>,
     force_smoothings: f64,
     delta_t: f64,
) -> Receiver<Vec<Object>> {
     let (tx, rx) = mpsc::channel();

     thread::spawn(move || engine_thread(objects, tx, force_smoothings, delta_t));

     rx
}
