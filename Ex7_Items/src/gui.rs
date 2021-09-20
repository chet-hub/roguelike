use bracket_terminal::prelude::{BTerm, RGB, YELLOW, RED, BLACK, WHITE};
use crate::{State, map};
use specs::{World, WorldExt, Join};
use crate::components::{Player, CombatStats, Position, Name, Item, InBackpack};
use crate::map::*;

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

    show_inventory(ecs,ctx);
}



pub fn show_inventory(ecs: &World, ctx: &mut BTerm){
    let items = ecs.read_storage::<Item>();
    let inBackpacks = ecs.read_storage::<InBackpack>();
    let names = ecs.read_storage::<Name>();
    let mut list:Vec<&str> = Vec::new();
    for(_item,_inBackpack,name) in (&items,&inBackpacks,&names).join() {
        list.push(&*name.name);
    }
    inventory_box(list,ctx);
}



pub fn inventory_box(list: Vec<&str>, ctx: &mut BTerm){
    //let list = vec!["11111111","2222222222","3333333333333"];
    let width = MAP_WIDTH/2;
    let high = list.len();
    let x = map::MAP_WIDTH/2 - width/2;
    let y = map::MAP_HEIGHT/2 - high/2;
    ctx.draw_box_double(x, y, map::MAP_WIDTH/2, high + 1, YELLOW, BLACK);
    ctx.print(x + 4, y,format!("Esc to close"));
    for (i,st) in list.iter().enumerate() {
        ctx.print(x+1 , y+1 + i,format!("({}) {}", i,st));
    }
}


//todo
pub fn drop_item_menu(){

}














