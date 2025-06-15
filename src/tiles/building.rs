use crate::placement_logic::building_matcher::match_building;
use crate::player::Player;
use crate::tiles::Tile;
use crate::tiles::resources::Resources;
use std::fmt;
pub struct Building {
    kind: BuildingType,
    shape: [[Tile; 4]; 2],
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum BuildingType {
    House = 0,
    Castle = 1,
}

pub const ALL_BUILDINGS: [BuildingType; 2] = [BuildingType::House, BuildingType::Castle];

impl BuildingType {
    pub fn to_string(&self) -> String {
        match self {
            BuildingType::House => String::from("House"),
            BuildingType::Castle => String::from("Castle"),
        }
    }

    pub fn string_to_building(tile: &str) -> BuildingType {
        match tile {
            "House" | "house" => BuildingType::House,
            "Castle" | "castle" => BuildingType::Castle,
            _ => BuildingType::House,
        }
    }
}

impl fmt::Display for BuildingType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            BuildingType::House => String::from("House"),
            BuildingType::Castle => String::from("Castle"),
            _ => String::from("Empty"),
        };
        write!(f, "{}", s)
    }
}

impl Building {
    pub fn new(building_type: BuildingType, shape: [[Tile; 4]; 2]) -> Building {
        Building {
            kind: building_type,
            shape: shape,
        }
    }

    pub fn get_kind(&self) -> BuildingType {
        self.kind
    }

    pub fn print_building(&self) {
        println!("Building:");
        for row in &self.shape {
            for cell in row {
                print!("{},", *cell);
            }
            print!("\n");
        }
    }

    pub fn print_building_90(&self) {
        println!("Building:");
        for col in 0..self.shape[0].len() {
            for row in (0..self.shape.len()).rev() {
                print!("{},", self.shape[row][col]);
            }
            print!("\n");
        }
    }

    pub fn print_building_180(&self) {
        println!("Building:");
        for row in (0..self.shape.len()).rev() {
            for col in (0..self.shape[0].len()).rev() {
                print!("{},", self.shape[row][col]);
            }
            print!("\n");
        }
    }

    pub fn print_building_270(&self) {
        println!("Building:");
        for col in (0..self.shape[0].len()).rev() {
            for row in 0..self.shape.len() {
                print!("{},", self.shape[row][col]);
            }
            print!("\n");
        }
    }

    pub fn get_shape(&self) -> [[Tile; 4]; 2] {
        self.shape
    }

    // pub fn last_non_empty_tile_col(&self) -> usize {
    //     let mut end = 0;
    //     for row in &self.shape {
    //         for col in 0..row.len() {
    //             if row[col] != 0 && col > end {
    //                 end = col;
    //             }
    //         }
    //     }
    //     return end;
    // }

    pub fn last_non_empty_tile_row(&self) -> usize {
        let mut end = 0;
        for row in 0..self.shape.len() {
            for col in 0..self.shape[0].len() {
                if !self.shape[row][col].is_empty() && row > end {
                    end = row;
                }
            }
        }
        return end;
    }

    pub fn print_shape(&self) {
        for row in &self.shape {
            for cell in row {
                print!("{},", *cell);
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
        player.place_tile(1, 1, Tile::Resource(Resources::Wheat));
        player.place_tile(1, 2, Tile::Resource(Resources::Brick));
        player.place_tile(1, 3, Tile::Resource(Resources::Glass));
        player.place_tile(1, 0, Tile::Resource(Resources::Stone));
        player.place_tile(0, 2, Tile::Resource(Resources::Brick));
        player.place_tile(0, 1, Tile::Resource(Resources::Glass));

        let building = Building::new(
            BuildingType::House,
            [
                [
                    Tile::Resource(Resources::Wheat),
                    Tile::Resource(Resources::Brick),
                    Tile::Resource(Resources::Glass),
                    Tile::Resource(Resources::Empty),
                ],
                [
                    Tile::Resource(Resources::Empty),
                    Tile::Resource(Resources::Empty),
                    Tile::Resource(Resources::Empty),
                    Tile::Resource(Resources::Empty),
                ],
            ],
        );
        let result = match_building(&player, &building);
        assert_eq!(result, true);
    }

    #[test]
    fn it_rotated_90_match() {
        let mut player: Player = Player::new();
        player.place_tile(1, 1, Tile::Resource(Resources::Wheat));
        player.place_tile(2, 1, Tile::Resource(Resources::Brick));
        player.place_tile(3, 1, Tile::Resource(Resources::Glass));
        player.place_tile(1, 0, Tile::Resource(Resources::Stone));
        player.place_tile(0, 2, Tile::Resource(Resources::Brick));
        player.place_tile(0, 1, Tile::Resource(Resources::Glass));

        let building = Building::new(
            BuildingType::House,
            [
                [
                    Tile::Resource(Resources::Wheat),
                    Tile::Resource(Resources::Brick),
                    Tile::Resource(Resources::Glass),
                    Tile::Resource(Resources::Empty),
                ],
                [
                    Tile::Resource(Resources::Empty),
                    Tile::Resource(Resources::Empty),
                    Tile::Resource(Resources::Empty),
                    Tile::Resource(Resources::Empty),
                ],
            ],
        );
        let result = match_building(&player, &building);
        assert_eq!(result, true);
    }

    #[test]
    fn it_rotated_180_match() {
        let mut player: Player = Player::new();
        player.place_tile(1, 3, Tile::Resource(Resources::Wheat));
        player.place_tile(1, 2, Tile::Resource(Resources::Brick));
        player.place_tile(1, 1, Tile::Resource(Resources::Glass));
        player.place_tile(1, 0, Tile::Resource(Resources::Stone));
        player.place_tile(0, 2, Tile::Resource(Resources::Brick));
        player.place_tile(0, 1, Tile::Resource(Resources::Glass));

        let building = Building::new(
            BuildingType::House,
            [
                [
                    Tile::Resource(Resources::Wheat),
                    Tile::Resource(Resources::Brick),
                    Tile::Resource(Resources::Glass),
                    Tile::Resource(Resources::Empty),
                ],
                [
                    Tile::Resource(Resources::Empty),
                    Tile::Resource(Resources::Empty),
                    Tile::Resource(Resources::Empty),
                    Tile::Resource(Resources::Empty),
                ],
            ],
        );
        let result = match_building(&player, &building);
        assert_eq!(result, true);
    }

    #[test]
    fn it_rotated_270_match() {
        let mut player: Player = Player::new();
        player.place_tile(3, 1, Tile::Resource(Resources::Wheat));
        player.place_tile(2, 1, Tile::Resource(Resources::Brick));
        player.place_tile(1, 1, Tile::Resource(Resources::Glass));
        player.place_tile(1, 0, Tile::Resource(Resources::Stone));
        player.place_tile(0, 2, Tile::Resource(Resources::Brick));
        player.place_tile(0, 1, Tile::Resource(Resources::Glass));

        let building = Building::new(
            BuildingType::House,
            [
                [
                    Tile::Resource(Resources::Wheat),
                    Tile::Resource(Resources::Brick),
                    Tile::Resource(Resources::Glass),
                    Tile::Resource(Resources::Empty),
                ],
                [
                    Tile::Resource(Resources::Empty),
                    Tile::Resource(Resources::Empty),
                    Tile::Resource(Resources::Empty),
                    Tile::Resource(Resources::Empty),
                ],
            ],
        );
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
        player.place_tile(1, 1, Tile::Resource(Resources::Wheat));
        player.place_tile(0, 1, Tile::Resource(Resources::Brick));
        player.place_tile(1, 0, Tile::Resource(Resources::Stone));
        player.place_tile(0, 2, Tile::Resource(Resources::Brick));

        let building = Building::new(
            BuildingType::House,
            [
                [
                    Tile::Resource(Resources::Wheat),
                    Tile::Resource(Resources::Brick),
                    Tile::Resource(Resources::Empty),
                    Tile::Resource(Resources::Empty),
                ],
                [
                    Tile::Resource(Resources::Empty),
                    Tile::Resource(Resources::Empty),
                    Tile::Resource(Resources::Empty),
                    Tile::Resource(Resources::Empty),
                ],
            ],
        );
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
        player.place_tile(1, 1, Tile::Resource(Resources::Wheat));
        player.place_tile(1, 0, Tile::Resource(Resources::Brick));

        let building = Building::new(
            BuildingType::House,
            [
                [
                    Tile::Resource(Resources::Wheat),
                    Tile::Resource(Resources::Brick),
                    Tile::Resource(Resources::Empty),
                    Tile::Resource(Resources::Empty),
                ],
                [
                    Tile::Resource(Resources::Empty),
                    Tile::Resource(Resources::Empty),
                    Tile::Resource(Resources::Empty),
                    Tile::Resource(Resources::Empty),
                ],
            ],
        );
        player.print_board();
        building.print_building();

        let result = match_building(&player, &building);
        assert_eq!(result, true);
    }

    #[test]
    fn it_rotated_90_match_edge() {
        let mut player: Player = Player::new();
        player.place_tile(2, 0, Tile::Resource(Resources::Wheat));
        player.place_tile(3, 0, Tile::Resource(Resources::Brick));

        let building = Building::new(
            BuildingType::House,
            [
                [
                    Tile::Resource(Resources::Wheat),
                    Tile::Resource(Resources::Brick),
                    Tile::Resource(Resources::Empty),
                    Tile::Resource(Resources::Empty),
                ],
                [
                    Tile::Resource(Resources::Empty),
                    Tile::Resource(Resources::Empty),
                    Tile::Resource(Resources::Empty),
                    Tile::Resource(Resources::Empty),
                ],
            ],
        );
        player.print_board();
        building.print_building();

        let result = match_building(&player, &building);
        assert_eq!(result, true);
    }
}
