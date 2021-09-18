use specs::prelude::System;
use specs::{Storage, WriteStorage, ReadStorage, Join, WriteExpect};
use crate::map::Map;
use crate::components::{Position, Renderable};
use bracket_terminal::prelude::{Point, BTerm};
use bracket_pathfinding::prelude::Algorithm2D;


pub struct RenderSystem {}

impl<'a> System<'a> for RenderSystem {
    type SystemData = (
        WriteExpect<'a, BTerm>,
        WriteExpect<'a, Box<Map>>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderable>
    );
    fn run(&mut self, data: Self::SystemData) {

        let (mut ctx,map, position, renderable) = data;
        ctx.cls();
        map.draw_map(&mut ctx);
        for (pos, render) in (&position, &renderable).join() {
            let idx = map.point2d_to_index(Point { x: pos.x, y: pos.y });
            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph)
            }
        }


    }
}