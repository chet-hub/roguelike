use specs::{World, WorldExt, Builder};
use crate::components::{Position, Renderable, Player, BlocksTile, CombatStats, Name, Viewshed, Monster, Item, Consumable};
use bracket_terminal::prelude::{RGB, YELLOW, BLACK, RED, to_cp437};
use crate::map::Map;
use bracket_pathfinding::prelude::{Point, Rect};
use bracket_random::prelude::RandomNumberGenerator;
use std::collections::{HashSet, HashMap};

pub fn spawn_player(ecs: &mut World, x: i32, y: i32) {
    let player_entity = ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
        })
        .with(Player {})
        .with(BlocksTile {})
        .with(CombatStats {
            max_hp: 100,
            hp: 100,
            defense: 5,
            power: 50,
        })
        .with(Name { name: "Hero".to_string() })
        .with(Viewshed { visible_tiles: Vec::new(), range: 9, dirty: true })
        .build();
    ecs.insert(Point::new(x, y));
    ecs.insert(player_entity);
}

pub fn spawn_monster(ecs: &mut World, x: i32, y: i32, i: i32) {
    ecs.create_entity()
        .with(Monster {})
        .with(Name { name: format!("Monster[{}]", i.to_string()) })
        .with(Viewshed { visible_tiles: Vec::new(), range: 9, dirty: true })
        .with(Renderable {
            glyph: to_cp437('M'),
            fg: RGB::named(RED),
            bg: RGB::named(BLACK),
        })
        .with(Position { x, y })
        .with(BlocksTile {})
        .with(CombatStats {
            max_hp: 10,
            hp: 10,
            defense: 5,
            power: 10,
        })
        .build();
}

pub fn spawn_item_potion(ecs: &mut World, x: i32, y: i32, i: i32) {
    ecs.create_entity()
        .with(Item {})
        .with(Name { name: format!("potion[{}]", i.to_string()) })
        .with(Viewshed { visible_tiles: Vec::new(), range: 9, dirty: true })
        .with(Renderable {
            glyph: to_cp437('!'),
            fg: RGB::named(RED),
            bg: RGB::named(BLACK),
        })
        .with(Position { x, y })
        .with(Consumable {
            hp: 50,
        })
        .build();
}

pub fn getRandomPointInRectangle(re: &Rect) -> (i32, i32) {
    let mut rng = RandomNumberGenerator::new();
    let x = rng.range(re.x1 + 1, re.x2);
    let y = rng.range(re.y1 + 1, re.y2);
    (x, y)
}

const MAX_MONSTERS: i32 = 9;
const MAX_ITEMS: i32 = 9;


pub fn spawning_to_map(ecs: &mut World, map: &mut Map) {
    let p = map.rooms[0].center();
    spawn_player(ecs, p.x, p.y);
    //ecs.insert(Point::new(p.x, p.y));

    let mut positions: HashMap<String, (i32, i32)> = HashMap::new();
    let mut count = MAX_MONSTERS + MAX_ITEMS;
    while positions.len() != count as usize {
        for (i, rec) in map.rooms.iter().skip(1).enumerate() {
            let p = getRandomPointInRectangle(rec);
            let p_key = p.0.to_string() + &*p.1.to_string();
            if !positions.contains_key(&p_key) {
                positions.insert(p_key, p);
            }
            if positions.len() == count as usize {
                break;
            }
        }
    }

    let mut count = 0;

    for (_k, v) in positions.iter().enumerate() {
        if count < MAX_ITEMS {
            spawn_item_potion(ecs, v.1.0, v.1.1, count);
        } else {
            spawn_monster(ecs, v.1.0, v.1.1, count);
        }
        count += 1;
    }
}


#[cfg(test)]
mod test {
    use bracket_random::prelude::RandomNumberGenerator;
    // 1 by 3 stripe of tiles

    #[test]
    fn test_dice() {
        let mut rng = RandomNumberGenerator::new();
        for i in 0..100 {
            let x = rng.range(0, 10);
            print!("{},", x)
        }
    }
}
