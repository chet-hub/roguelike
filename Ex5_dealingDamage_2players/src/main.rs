mod player;

use player::*;

mod components;

use components::*;

mod map;
mod visibility_system;
mod monster_ai_system;
mod render_system;
mod MapIndexing_system;
mod player_move_system;
mod turnState;
mod input_polling;
mod input_system;

use visibility_system::VisibilitySystem;
use bracket_pathfinding::prelude::*;
use bracket_random::prelude::*;
use bracket_terminal::prelude::*;
use specs_derive::*;
use specs::prelude::*;
use std::cmp::{max, min};
use crate::turnState::{TurnState};
use crate::map::Map;
use crate::monster_ai_system::MonsterAI;
use crate::render_system::RenderSystem;
use crate::MapIndexing_system::MapIndexingSystem;
use crate::player_move_system::PlayerMoveSystem;
use crate::input_polling::*;
use specs::shred::FetchMut;
use crate::input_system::InputSystem;


pub struct State {
    ecs: World,
    context: BTerm,
    turnState: TurnState,
}

impl State {
    pub fn new() -> State {
        let mut context = BTermBuilder::simple80x50()
            .with_title("Roguelike")
            .build().unwrap();

        return State {
            ecs: World::new(),
            context,
            turnState: TurnState::new(),
        };
    }

    pub fn init(&mut self) {
        self.ecs.insert(self.context.clone());


        let map = Map::new_map_rooms_and_corridors();
        let p = map.rooms[0].center();
        self.ecs.insert(Point::new(p.x, p.y));


        for (i, rec) in map.rooms.iter().skip(1).enumerate() {
            let Point { x, y } = rec.center();
            let monster_entity = self.ecs.create_entity()
                .with(Monster {})
                .with(Name { name: i.to_string() })
                .with(Viewshed { visible_tiles: Vec::new(), range: 9, dirty: true })
                .with(Renderable {
                    glyph: to_cp437('M'),
                    fg: RGB::named(RED),
                    bg: RGB::named(BLACK),
                })
                .with(Position { x, y })
                .with(InputComponent::new())
                .with(BlocksTile {})
                .build();
            //println!("===add_monsters--->{:?}",monster_entity.clone());
            //self.turnState.add_player(monster_entity.clone(), get_input_for_AI);
        }

        let boxMap = Box::new(map);
        self.ecs.insert(boxMap);

        let player_entity = self.ecs.create_entity()
            .with(Position { x: p.x, y: p.y })
            .with(Renderable {
                glyph: to_cp437('@'),
                fg: RGB::named(YELLOW),
                bg: RGB::named(BLACK),
            })
            .with(Player {})
            .with(InputComponent::new())
            .with(BlocksTile {})
            .with(Name { name: "Hero".to_string() })
            .with(Viewshed { visible_tiles: Vec::new(), range: 9, dirty: true })
            .build();
        self.ecs.insert(player_entity.clone());

        println!("===add_player--->{:?}", player_entity.clone());
        self.turnState.add_player(player_entity.clone(), get_input_from_keyboard_and_moues);


        let player_entity = self.ecs.create_entity()
            .with(Position { x: p.x + 1, y: p.y + 1 })
            .with(Renderable {
                glyph: to_cp437('@'),
                fg: RGB::named(YELLOW),
                bg: RGB::named(BLACK),
            })
            .with(Player {})
            .with(InputComponent::new())
            .with(BlocksTile {})
            .with(Name { name: "Hero".to_string() })
            .with(Viewshed { visible_tiles: Vec::new(), range: 9, dirty: true })
            .build();
        self.ecs.insert(player_entity.clone());

        println!("===add_player--->{:?}", player_entity.clone());
        self.turnState.add_player(player_entity.clone(), get_input_from_keyboard_and_moues);
    }

    pub fn register_component(&mut self) {
        self.ecs.register::<InputComponent>();
        self.ecs.register::<BlocksTile>();
        self.ecs.register::<CombatStats>();
        self.ecs.register::<SufferDamage>();
        self.ecs.register::<Position>();
        self.ecs.register::<Renderable>();
        self.ecs.register::<Player>();
        self.ecs.register::<Viewshed>();
        self.ecs.register::<Monster>();
        self.ecs.register::<Name>();
    }

    fn run_systems(&mut self) {
        VisibilitySystem {}.run_now(&self.ecs);
        MonsterAI {}.run_now(&self.ecs);
        MapIndexingSystem {}.run_now(&self.ecs);
        PlayerMoveSystem {}.run_now(&self.ecs);
        RenderSystem {}.run_now(&self.ecs);

        self.ecs.maintain();
    }

    fn start(self) -> BResult<()> {
        main_loop(self.context.clone(), self)
    }
}


impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.ecs.maintain();

        //send
        let optionComps: Option<InputComponent> = self.turnState.poll_input(ctx);
        let entity = self.turnState.current_turn();
        //println!("===current_turn--->{:?}",entity);
        if let Some(mut co) = optionComps {
            co.accept_input = true;
            {
                let mut inputStorage = self.ecs.write_component::<InputComponent>();
                println!("send--->{:?}, {:?}", entity, co);
                inputStorage.insert(entity.clone(), co);
            }
        } else {
            return;
        }

        //run once
        //self.run_systems();
        InputSystem {}.run_now(&self.ecs);
        MonsterAI {}.run_now(&self.ecs);
        PlayerMoveSystem {}.run_now(&self.ecs);
        VisibilitySystem {}.run_now(&self.ecs);
        MapIndexingSystem {}.run_now(&self.ecs);
        RenderSystem {}.run_now(&self.ecs);


        //check

        {
            let mut inputStorage = self.ecs.read_component::<InputComponent>();
            let value = inputStorage.get(entity.clone());

            if let Some(t) = value {
                if t.accept_input == false {
                    self.turnState.next_turn();
                }
            } else {
                self.turnState.remove_current_player();
            }
        }
    }
}


fn main() -> BError {
    let mut gs = State::new();
    gs.register_component();
    gs.init();
    gs.start()
}