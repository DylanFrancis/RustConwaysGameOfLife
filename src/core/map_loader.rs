use specs::World;
use crate::components::Position;
use crate::entities::create_live_cell;

pub fn load_map(world: &mut World, map: String) {
    let start = u128::max_value() / 2;

    let mut row: u128 = start;
    let mut col: u128 = start;

    for char in map.chars() {
        match char {
            '\n' => {
                row += 1;
                col = start;
                continue;
            }
            'x' => {
                let pos = Position {
                    x: col,
                    y: row
                };
                println!("{:?}", pos);
                create_live_cell(world, pos);
                col += 1;
            }
            '.' => { col += 1; }
            _ => {}
        }
    }
}