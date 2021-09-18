use specs::{World, WorldExt, Builder};
use crate::components::{Position, Renderable, Player, BlocksTile, CombatStats, Name, Viewshed, Monster};
use bracket_terminal::prelude::{RGB, YELLOW, BLACK, RED, to_cp437};
use crate::map::Map;
use bracket_pathfinding::prelude::{Point, Rect};
use bracket_random::prelude::RandomNumberGenerator;
use std::collections::{HashSet, HashMap};

pub fn spawn_player(ecs: &mut World, x: i32, y: i32) {
    let player_entity0 = ecs.create_entity()
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

pub fn spawn_item(ecs: &mut World, x: i32, y: i32, i: i32) {
    ecs.create_entity()
        // .with(Monster {})
        // .with(Name { name: format!("Monster[{}]",i.to_string()) })
        // .with(Viewshed { visible_tiles: Vec::new(), range: 9, dirty: true })
        // .with(Renderable {
        //     glyph: to_cp437('M'),
        //     fg: RGB::named(RED),
        //     bg: RGB::named(BLACK),
        // })
        // .with(Position { x, y })
        // .with(BlocksTile {})
        // .with(CombatStats {
        //     max_hp: 10,
        //     hp: 10,
        //     defense: 5,
        //     power: 10,
        // })
        .build();
}

pub fn getRandomPointInRectangle(re: &Rect) -> (i32, i32) {
    let mut rng = RandomNumberGenerator::new();
    let x = rng.range(re.x1 + 1, re.x2);
    let y = rng.range(re.y1 + 1, re.y2);
    (x, y)
}

const MAX_MONSTERS: i32 = 4;
const MAX_ITEMS: i32 = 2;


pub fn spawning_to_map(ecs: &mut World, map: &mut Map) {
    let p = map.rooms[0].center();
    spawn_player(ecs, p.x, p.y);
    //ecs.insert(Point::new(p.x, p.y));

    let mut positions: HashMap<str, (i32, i32)> = HashMap::new();
    let mut count = MAX_MONSTERS + MAX_ITEMS;
    while positions.len() != count as usize {
        for (i, rec) in map.rooms.iter().skip(1).enumerate() {
            let p = getRandomPointInRectangle(rec);
            let p_key = p.x.to_String() + p.y.to_String();
            if !positions.contains_key(&p_key) {
                positions.insert(p_key, p);
            }
            if positions.len() == count as usize {
                break;
            }
        }
    }

    let mut count = 0;

    for (k, v) in positions.iter().enumerate() {
        if count < MAX_ITEMS {
            spawn_item(ecs, x, y, i as i32);
        } else {
            spawn_monster(ecs, x, y, i as i32);
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
