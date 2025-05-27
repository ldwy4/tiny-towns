use crate::tiles::Tiles;
use crate::player::Player;
pub struct Building {
    kind: BuildingType,
    shape: [[u8; 4]; 2],
    secondFloor: bool, // true if building has a second floor
}

pub enum BuildingType {
    House,
    Castle,
}

impl Building {
    pub fn new(building_type: BuildingType, shape: [[u8; 4]; 2], secondFloor: bool) -> Building {
        Building {
            kind: building_type,
            shape: shape,
            secondFloor: secondFloor,
        }
    }

    pub fn print_building(&self) {
        println!("Building:");
        for row in &self.shape {
            for cell in row {
                print!("{},", Tiles::number_to_tile(*cell));
            }
            print!("\n");
        }
    }

    pub fn get_shape(&self) -> [[u8; 4]; 2] {
        self.shape
    }

    pub fn get_second_floor(&self) -> bool {
        self.secondFloor
    }

    pub fn last_non_empty_tile_col(&self) -> usize {
        let mut end = 0;
        for row in &self.shape {
            for col in 0..row.len() {
                if row[col] != 0 && col > end {
                    end = col;
                }
            }
        }
        return end;
    }

    pub fn last_non_empty_tile_row(&self) -> usize {
        let mut end = 0;
        for row in 0..self.shape.len() {
            for col in 0..self.shape[0].len() {
                if self.shape[row][col] != 0 && row > end {
                    end = row;
                }
            }
        }
        return end;
    }

    pub fn print_shape(&self) {
        for row in &self.shape {
            for cell in row {
                print!("{},", Tiles::number_to_tile(*cell));
            }
            print!("\n");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut player: Player = Player::new();
        player.place_tile(1, 1, 1);
        player.place_tile(2, 1, 2);
        player.place_tile(3, 1, 3);
        player.place_tile(1, 0, 4);
        player.place_tile(0, 2, 2);
        player.place_tile(0, 1, 3);

        let building = Building::new(BuildingType::House, [[1, 2, 3, 0], [0, 0, 0, 0]], false);
        let result = player.match_building(building);
        assert_eq!(result, true);
    }
}