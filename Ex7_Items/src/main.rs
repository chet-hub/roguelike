mod player;

use player::*;

mod components;

use components::*;

mod map;
mod visibility_system;
mod monster_ai_system;
mod render_system;
mod MapIndexing_system;
mod melee_combat_system;
mod damage_system;
mod gui;
mod spawning;

use visibility_system::VisibilitySystem;
use bracket_pathfinding::prelude::*;
use bracket_random::prelude::*;
use bracket_terminal::prelude::*;
use specs_derive::*;
use specs::prelude::*;
use std::cmp::{max, min};
use crate::map::Map;
use crate::monster_ai_system::MonsterAI;
use crate::render_system::RenderSystem;
use crate::MapIndexing_system::MapIndexingSystem;
use specs::shred::FetchMut;
use crate::damage_system::DamageSystem;
use crate::melee_combat_system::MeleeCombatSystem;
use crate::gui::GameLog;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState { AwaitingInput, PreRun, PlayerTurn, MonsterTurn }

pub struct State {
    ecs: World,
    context: BTerm,
}

impl State {
    pub fn new() -> State {
        let mut context = BTermBuilder::simple80x50()
            .with_title("Roguelike")
            .build().unwrap();

        return State {
            ecs: World::new(),
            context,
        };
    }

    pub fn init(&mut self) {
        self.ecs.insert(self.context.clone());
        let mut map = Map::new_map_rooms_and_corridors();
        spawning::spawning_to_map(&mut self.ecs, &mut map);
        self.ecs.insert(Box::new(map));
        self.ecs.insert(RunState::PreRun);
        self.ecs.insert(GameLog { entries: vec!["Welcome to Rusty Roguelike".to_string()] });
    }

    pub fn register_component(&mut self) {
        self.ecs.register::<BlocksTile>();
        self.ecs.register::<CombatStats>();
        self.ecs.register::<SufferDamage>();
        self.ecs.register::<Position>();
        self.ecs.register::<Renderable>();
        self.ecs.register::<Player>();
        self.ecs.register::<Viewshed>();
        self.ecs.register::<Monster>();
        self.ecs.register::<Name>();
        self.ecs.register::<WantsToMelee>();
    }

    fn run_systems(&mut self) {

        VisibilitySystem {}.run_now(&self.ecs);
        DamageSystem {}.run_now(&self.ecs);
        MeleeCombatSystem {}.run_now(&self.ecs);
        MonsterAI {}.run_now(&self.ecs);
        MapIndexingSystem {}.run_now(&self.ecs);



        self.ecs.maintain();
    }

    fn start(self) -> BResult<()> {
        main_loop(self.context.clone(), self)
    }
}


impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let mut newrunstate;
        {
            let runstate = self.ecs.fetch::<RunState>();
            newrunstate = *runstate;
        }

        match newrunstate {
            RunState::PreRun => {
                self.run_systems();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::AwaitingInput => {
                newrunstate = player_input(self, ctx);
            }
            RunState::PlayerTurn => {
                self.run_systems();
                newrunstate = RunState::MonsterTurn;
            }
            RunState::MonsterTurn => {
                self.run_systems();
                newrunstate = RunState::AwaitingInput;
            }
        }

        {
            let mut runwriter = self.ecs.write_resource::<RunState>();
            *runwriter = newrunstate;
        }

        RenderSystem {}.run_now(&self.ecs);

        gui::draw_tooltips(&self.ecs, ctx);
        gui::draw_ui(&self.ecs, ctx);
    }
}


fn main() -> BError {
    let mut gs = State::new();
    gs.register_component();
    gs.init();
    gs.start()
}