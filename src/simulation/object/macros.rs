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

macro_rules! create_object_value_checked {
     ($mass: expr, $loc: expr, $can_move: expr, $color: expr) => {
          Object::new_inactive($mass, $loc, $can_move, $color)
               .map_err(|e| anyhow::anyhow!(e))
               .with_context(|| "Couldn't create object.")?
     };
}

pub(crate) use create_object_value_checked;
