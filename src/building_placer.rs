use crate::building_matcher::verify_building;
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
) {
    // if building is valid, replace resources with building tile
    if verify_building(player, building, placement_row, placement_col) {
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
    } else {
        println!("Invalid building placement");
    }
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
            false,
        );
        player.print_board();
        place_building(&mut player, &building, 1, 1);
        player.print_board();
        assert_eq!(
            player.get_board()[1][1] == Tile::Building(BuildingType::House),
            true
        );
        assert_eq!(player.get_board()[1][2].is_empty(), true);
        assert_eq!(player.get_board()[1][3].is_empty(), true);
    }
}
