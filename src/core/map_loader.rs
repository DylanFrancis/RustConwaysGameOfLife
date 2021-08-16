use specs::World;
use crate::components::Position;
use crate::entities::create_live_cell;

pub fn load_map(world: &mut World, map: String) {
    let mut row: i64 = 0;
    let mut col: i64 = 0;

    for char in map.chars() {
        let pos = Position {
            x: col,
            y: row
        };

        match char {
            '\n' => {
                row += 1;
                col = 0;
            }
            'x' => {
                create_live_cell(world, pos);
            }
            _ => {}
        }
        col += 1;
    }
}