mod player;

use player::*;

mod components;

use components::*;

mod map;
mod visibility_system;
mod monster_ai_system;

use visibility_system::VisibilitySystem;
use bracket_pathfinding::prelude::*;
use bracket_random::prelude::*;
use bracket_terminal::prelude::*;
use specs_derive::*;
use specs::prelude::*;
use std::cmp::{max, min};
use crate::map::Map;
use crate::monster_ai_system::MonsterAI;


#[derive(PartialEq, Copy, Clone)]
pub enum RunState { Paused, Running }

pub struct State {
    ecs: World,
    pub runstate: RunState,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        let mut mob = MonsterAI {};
        mob.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        if self.runstate == RunState::Running {
            self.run_systems();
            self.runstate = RunState::Paused;
        } else {
            self.runstate = player_input(self, ctx);
        }

        //draw map
        let map = self.ecs.fetch::<Map>();
        map.draw_map(ctx);

        //draw other entity with Position and renderable components
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        for (pos, render) in (&positions, &renderables).join() {
            let idx = map.point2d_to_index(Point { x: pos.x, y: pos.y });
            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph)
            }
        }
    }
}


fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Roguelike")
        .build()?;

    let mut gs = State {
        ecs: World::new(),
        runstate: RunState::Running,
    };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();


    let map = Map::new_map_rooms_and_corridors();
    let p = map.rooms[0].center();


    for (i, rec) in map.rooms.iter().skip(1).enumerate() {
        let Point { x, y } = rec.center();
        gs.ecs.create_entity()
            .with(Monster {})
            .with(Name { name: i.to_string() })
            .with(Viewshed { visible_tiles: Vec::new(), range: 9, dirty: true })
            .with(Renderable {
                glyph: to_cp437('M'),
                fg: RGB::named(RED),
                bg: RGB::named(BLACK),
            })
            .with(Position { x, y })
            .build();
    }

    gs.ecs.create_entity()
        .with(Position { x: p.x, y: p.y })
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
        })
        .with(Player {})
        .with(Name { name: "Hero".to_string() })
        .with(Viewshed { visible_tiles: Vec::new(), range: 9, dirty: true })
        .build();

    gs.ecs.insert(map);
    gs.ecs.insert(Point::new(p.x, p.y));

    main_loop(context, gs)
}