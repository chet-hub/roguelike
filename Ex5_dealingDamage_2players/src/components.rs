use bracket_terminal::prelude::*;
use specs::prelude::*;
use specs_derive::*;

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component, Debug)]
pub struct Player {}

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<Point>,
    pub range: i32,
    pub dirty: bool,
}

#[derive(Component, Debug)]
pub struct Monster {}

#[derive(Component, Debug)]
pub struct Name {
    pub name: String,
}

#[derive(Component, Debug)]
pub struct BlocksTile {}


#[derive(Component, Debug)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}

#[derive(Component, Debug)]
pub struct SufferDamage {
    pub amount: Vec<i32>,
}

#[derive(Component, Debug)]
pub struct InputComponent {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub left_up: bool,
    pub right_up: bool,
    pub left_down: bool,
    pub right_down: bool,

    pub accept_input:bool
}

impl InputComponent {
    pub(crate) fn new() -> InputComponent {
        InputComponent{
            left: false,
            right: false,
            up: false,
            down: false,
            left_up: false,
            right_up: false,
            left_down: false,
            right_down: false,

            accept_input: false
        }
    }
}
