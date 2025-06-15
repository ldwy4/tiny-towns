use crate::placement_logic::building_matcher::{
    verify_building, verify_building_90_degrees, verify_building_180_degrees,
    verify_building_270_degrees,
};
use crate::player::Player;
use crate::tiles::Building;
use crate::tiles::BuildingType;
use crate::tiles::Resources;
use crate::tiles::Tile;
/*
 * Functions for placing a building on the board and removing the resource tiles used to create the building
 */

/*
 * Place a building on the board
 */
pub fn place_building(
    player: &mut Player,
    building: &Building,
    placement_row: usize,
    placement_col: usize,
    rotation: usize,
) {
    match rotation {
        0 => verify_building_and_place(player, building, placement_row, placement_col),
        1 => verify_building_and_place_90(player, building, placement_row, placement_col),
        2 => verify_building_and_place_180(player, building, placement_row, placement_col),
        3 => verify_building_and_place_270(player, building, placement_row, placement_col),
        _ => println!("Invalid rotation"),
    }
}

pub fn verify_building_and_place(
    player: &mut Player,
    building: &Building,
    placement_row: usize,
    placement_col: usize,
) {
    if verify_building(player, building, placement_row, placement_col) {
        replace_resources_with_building(player, building, placement_row, placement_col);
    } else {
        println!("Invalid building placement");
    }
}

pub fn verify_building_and_place_90(
    player: &mut Player,
    building: &Building,
    placement_row: usize,
    placement_col: usize,
) {
    if verify_building_90_degrees(player, building, placement_row, placement_col) {
        replace_resources_with_building_90(player, building, placement_row, placement_col);
    } else {
        println!("Invalid building placement");
    }
}

pub fn verify_building_and_place_180(
    player: &mut Player,
    building: &Building,
    placement_row: usize,
    placement_col: usize,
) {
    if verify_building_180_degrees(player, building, placement_row, placement_col) {
        replace_resources_with_building_180(player, building, placement_row, placement_col);
    } else {
        println!("Invalid building placement");
    }
}

pub fn verify_building_and_place_270(
    player: &mut Player,
    building: &Building,
    placement_row: usize,
    placement_col: usize,
) {
    if verify_building_270_degrees(player, building, placement_row, placement_col) {
        replace_resources_with_building_270(player, building, placement_row, placement_col);
    } else {
        println!("Invalid building placement");
    }
}

pub fn replace_resources_with_building(
    player: &mut Player,
    building: &Building,
    placement_row: usize,
    placement_col: usize,
) {
    for row in 0..building.get_shape().len() {
        for col in 0..building.get_shape()[0].len() {
            let tile = building.get_shape()[row][col];
            if !tile.is_empty() {
                player.place_tile(
                    placement_row + row,
                    placement_col + col,
                    Tile::Resource(Resources::Empty),
                );
            }
        }
    }
    // place building tile after resources removed
    player.place_tile(
        placement_row,
        placement_col,
        Tile::Building(building.get_kind()),
    );
}

pub fn replace_resources_with_building_90(
    player: &mut Player,
    building: &Building,
    placement_row: usize,
    placement_col: usize,
) {
    for col in 0..building.get_shape()[0].len() {
        for row in (0..building.get_shape().len()).rev() {
            let tile = building.get_shape()[row][col];
            if !tile.is_empty() {
                player.place_tile(
                    placement_row + col,
                    placement_col - row,
                    Tile::Resource(Resources::Empty),
                );
            }
        }
    }
    // place building tile after resources removed
    player.place_tile(
        placement_row,
        placement_col,
        Tile::Building(building.get_kind()),
    );
}

pub fn replace_resources_with_building_180(
    player: &mut Player,
    building: &Building,
    placement_row: usize,
    placement_col: usize,
) {
    for row in (0..building.get_shape().len()).rev() {
        for col in (0..building.get_shape()[0].len()).rev() {
            let tile = building.get_shape()[row][col];
            if !tile.is_empty() {
                player.place_tile(
                    placement_row - row,
                    placement_col - col,
                    Tile::Resource(Resources::Empty),
                );
            }
        }
    }
    // place building tile after resources removed
    player.place_tile(
        placement_row,
        placement_col,
        Tile::Building(building.get_kind()),
    );
}

pub fn replace_resources_with_building_270(
    player: &mut Player,
    building: &Building,
    placement_row: usize,
    placement_col: usize,
) {
    for col in (0..building.get_shape()[0].len()).rev() {
        for row in 0..building.get_shape().len() {
            let tile = building.get_shape()[row][col];
            if !tile.is_empty() {
                player.place_tile(
                    placement_row - col,
                    placement_col + row,
                    Tile::Resource(Resources::Empty),
                );
            }
        }
    }
    // place building tile after resources removed
    player.place_tile(
        placement_row,
        placement_col,
        Tile::Building(building.get_kind()),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_place_building() {
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
        player.print_board();
        place_building(&mut player, &building, 1, 1, 0);
        player.print_board();
        assert_eq!(
            player.get_board()[1][1] == Tile::Building(BuildingType::House),
            true
        );
        assert_eq!(player.get_board()[1][2].is_empty(), true);
        assert_eq!(player.get_board()[1][3].is_empty(), true);
    }

    #[test]
    fn test_place_building_90() {
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
        player.print_board();
        place_building(&mut player, &building, 1, 1, 1);
        player.print_board();
        assert_eq!(
            player.get_board()[1][1] == Tile::Building(BuildingType::House),
            true
        );
        assert_eq!(player.get_board()[2][1].is_empty(), true);
        assert_eq!(player.get_board()[3][1].is_empty(), true);
    }

    #[test]
    fn test_place_building_180() {
        let mut player: Player = Player::new();
        player.place_tile(0, 3, Tile::Resource(Resources::Wheat));
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
        player.print_board();
        place_building(&mut player, &building, 0, 3, 2);
        player.print_board();
        assert_eq!(
            player.get_board()[0][3] == Tile::Building(BuildingType::House),
            true
        );
        assert_eq!(player.get_board()[0][2].is_empty(), true);
        assert_eq!(player.get_board()[0][1].is_empty(), true);
    }

    #[test]
    fn test_place_building_270() {
        let mut player: Player = Player::new();
        player.place_tile(3, 0, Tile::Resource(Resources::Wheat));
        player.place_tile(2, 0, Tile::Resource(Resources::Brick));
        player.place_tile(1, 0, Tile::Resource(Resources::Glass));

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
        player.print_board();
        place_building(&mut player, &building, 3, 0, 3);
        player.print_board();
        assert_eq!(
            player.get_board()[3][0] == Tile::Building(BuildingType::House),
            true
        );
        assert_eq!(player.get_board()[2][0].is_empty(), true);
        assert_eq!(player.get_board()[1][0].is_empty(), true);
    }
}
