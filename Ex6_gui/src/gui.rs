use bracket_terminal::prelude::{BTerm, RGB, YELLOW, RED, BLACK, WHITE};
use crate::State;
use specs::{World, WorldExt, Join};
use crate::components::{Player, CombatStats, Position, Name};
use crate::map::Map;

pub struct GameLog {
    pub entries : Vec<String>
}

pub fn draw_tooltips(ecs: &World, ctx: &mut BTerm){
    let mouse_pos = ctx.mouse_pos();
    // ctx.set_bg(mouse_pos.0, mouse_pos.1, WHITE);
    //let map = ecs.fetch::<Box<Map>>();

    let names = ecs.read_storage::<Name>();
    let positions = ecs.read_storage::<Position>();

    for(nameComponent,position) in (&names,&positions).join(){
        if position.x == mouse_pos.0 && position.y == mouse_pos.1 {
            //ctx.print_right(mouse_pos.0,mouse_pos.1,nameComponent.name.to_string());
            ctx.draw_box(mouse_pos.0, mouse_pos.1 + 1, nameComponent.name.to_string().len() + 2, 2, WHITE, BLACK);
            ctx.print(mouse_pos.0 + 1,mouse_pos.1 + 2,nameComponent.name.to_string());
        }
    }
}



pub fn draw_ui(ecs: &World, ctx: &mut BTerm) {
    ctx.draw_box(0, 43, 79, 6, WHITE, BLACK);

    let players = ecs.read_storage::<Player>();
    let stats = ecs.read_storage::<CombatStats>();
    for(_,status) in (&players,&stats).join(){
        ctx.draw_bar_horizontal(28, 43, 51, status.hp, status.max_hp, RED, BLACK);
    }

    let log = ecs.fetch::<GameLog>();
    let mut y = 44;
    for s in log.entries.iter().rev() {
        if y < 49 { ctx.print(2, y, s); }
        y += 1;
    }
}