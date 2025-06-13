use crate::player::Player;
use crate::tiles::Tiles;
use crate::building_matcher::match_building;
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

    pub fn print_building_90(&self) {
        println!("Building:");
        for col in 0..self.shape[0].len() {
            for row in (0..self.shape.len()).rev() {
                print!("{},", Tiles::number_to_tile(self.shape[row][col]));
            }
            print!("\n");
        }
    }

    pub fn print_building_180(&self) {
        println!("Building:");
        for row in (0..self.shape.len()).rev() {
            for col in (0..self.shape[0].len()).rev() {
                print!("{},", Tiles::number_to_tile(self.shape[row][col]));
            }
            print!("\n");
        }
    }

    pub fn print_building_270(&self) {
        println!("Building:");
        for col in (0..self.shape[0].len()).rev() {
            for row in 0..self.shape.len() {
                print!("{},", Tiles::number_to_tile(self.shape[row][col]));
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
    fn it_rotated_0_match() {
        let mut player: Player = Player::new();
        player.place_tile(1, 1, 1);
        player.place_tile(1, 2, 2);
        player.place_tile(1, 3, 3);
        player.place_tile(1, 0, 4);
        player.place_tile(0, 2, 2);
        player.place_tile(0, 1, 3);

        let building = Building::new(BuildingType::House, [[1, 2, 3, 0], [0, 0, 0, 0]], false);
        let result = match_building(&player, &building);
        assert_eq!(result, true);
    }

    #[test]
    fn it_rotated_90_match() {
        let mut player: Player = Player::new();
        player.place_tile(1, 1, 1);
        player.place_tile(2, 1, 2);
        player.place_tile(3, 1, 3);
        player.place_tile(1, 0, 4);
        player.place_tile(0, 2, 2);
        player.place_tile(0, 1, 3);

        let building = Building::new(BuildingType::House, [[1, 2, 3, 0], [0, 0, 0, 0]], false);
        let result = match_building(&player, &building);
        assert_eq!(result, true);
    }

    #[test]
    fn it_rotated_180_match() {
        let mut player: Player = Player::new();
        player.place_tile(1, 3, 1);
        player.place_tile(1, 2, 2);
        player.place_tile(1, 1, 3);
        player.place_tile(1, 0, 4);
        player.place_tile(0, 2, 2);
        player.place_tile(0, 1, 3);

        let building = Building::new(BuildingType::House, [[1, 2, 3, 0], [0, 0, 0, 0]], false);
        let result = match_building(&player, &building);
        assert_eq!(result, true);
    }

    #[test]
    fn it_rotated_270_match() {
        let mut player: Player = Player::new();
        player.place_tile(3, 1, 1);
        player.place_tile(2, 1, 2);
        player.place_tile(1, 1, 3);
        player.place_tile(1, 0, 4);
        player.place_tile(0, 2, 2);
        player.place_tile(0, 1, 3);

        let building = Building::new(BuildingType::House, [[1, 2, 3, 0], [0, 0, 0, 0]], false);
        building.print_building();
        building.print_building_90();
        building.print_building_180();
        building.print_building_270();
        player.print_board();
        let result = match_building(&player, &building);
        assert_eq!(result, true);
    }

    #[test]
    fn upright_two_resource_building() {
        let mut player: Player = Player::new();
        player.place_tile(1, 1, 1);
        player.place_tile(0, 1, 2);
        player.place_tile(1, 0, 4);
        player.place_tile(0, 2, 2);

        let building = Building::new(BuildingType::House, [[1, 2, 0, 0], [0, 0, 0, 0]], false);
        building.print_building();
        building.print_building_90();
        building.print_building_180();
        building.print_building_270();
        player.print_board();
        let result = match_building(&player, &building);
        assert_eq!(result, true);
    }

    #[test]
    fn it_rotated_180_match_edge() {
        let mut player: Player = Player::new();
        player.place_tile(1, 1, 1);
        player.place_tile(1, 0, 2);

        let building = Building::new(BuildingType::House, [[1, 2, 0, 0], [0, 0, 0, 0]], false);

        player.print_board();
        building.print_building();

        let result = match_building(&player, &building);
        assert_eq!(result, true);
    }

    #[test]
    fn it_rotated_90_match_edge() {
        let mut player: Player = Player::new();
        player.place_tile(2, 0, 1);
        player.place_tile(3, 0, 2);

        let building = Building::new(BuildingType::House, [[1, 2, 0, 0], [0, 0, 0, 0]], false);

        player.print_board();
        building.print_building();

        let result = match_building(&player, &building);
        assert_eq!(result, true);
    }
}
