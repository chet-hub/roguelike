mod player;
use player::*;
mod components;
use components::*;
mod map;
mod rect;

use bracket_pathfinding::prelude::*;
use bracket_random::prelude::*;
use bracket_terminal::prelude::*;
use specs_derive::*;
use specs::prelude::*;
use std::cmp::{max, min};

pub struct State {
    ecs: World,
}


impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        player::player_input(self, ctx);
        self.ecs.maintain();

        let map = self.ecs.fetch::<Vec<map::TileType>>();
        map::draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph)
        }
    }
}


fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Roguelike")
        .build()?;

    let mut gs = State {
        ecs: World::new()
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    let (rooms, map) = map::new_map_rooms_and_corridors();
    gs.ecs.insert(map);
    let p:Position = rooms[0].center();

    gs.ecs.create_entity()
        .with(p)
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
        })
        .with(Player {})
        .build();

    main_loop(context, gs)
}