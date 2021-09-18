use specs::{System, ReadStorage, join, Join, WriteStorage, WriteExpect, ReadExpect, Entity, Entities};
use crate::map::*;
use bracket_pathfinding::prelude::Point;
use crate::components::{Viewshed, Player, Position, InputComponent};
use std::cmp::{min, max};
use bracket_terminal::prelude::*;

pub struct InputSystem {}


impl<'a> System<'a> for InputSystem {
    type SystemData = (
        WriteStorage<'a, InputComponent>,
        Entities<'a>
    );


    fn run(&mut self, data: Self::SystemData) {
        let (
            mut inputComponent,
            mut entity
        ) = data;

        for (input, en) in (&mut inputComponent, &entity).join() {
            println!("XXXXXX-{:?},{:?}", en.id(),input);
        }
    }
}