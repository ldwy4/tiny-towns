use std::collections::btree_map::Range;
use std::iter::Rev;

use crate::building::{Building, BuildingType};
use crate::tiles::Tiles;

const BOARD_SIZE: usize = 4;
// TODO: change be a variable passed in on game creation
const BUILDING_TYPE_IN_GAME: usize = 7;

pub struct Player {
    board: [[u8; BOARD_SIZE]; BOARD_SIZE],
    score: i32,
}

pub enum TileType {
    BuildingType(BuildingType),
    Resource(u8),
}

/*
 * Contains the resource/building currently placed on the spot.
 *
 * If a resource, then it contains list of all building that could be
 * placed at this location.
 *
 * If a building, then list is empty
 *
 */
pub struct PlayerTileInfo {
    tile: TileType,
    possible_buildings: [BuildingType; BUILDING_TYPE_IN_GAME],
}

/*
 * For placing a building:
 *
 * GUI idea: click, drag, and confirm placement
 *
 * Verify that all the tiles in desired placement contain building in
 * the tile info
 *
 * - Remove Empty tiles from GUI
 *
 * - MVP, get someone to place a building
 * - start cli implementation
 * - allow users to input to cli resource type and location on board
 *
 */

impl Player {
    pub fn new() -> Player {
        Player {
            board: [[0; BOARD_SIZE]; BOARD_SIZE],
            score: 0,
        }
    }

    pub fn print_board(&self) {
        // implementation to print the board
        println!("Board:");
        for row in &self.board {
            for cell in row {
                print!("{},", Tiles::number_to_tile(*cell));
            }
            print!("\n");
        }
    }

    pub fn place_tile(&mut self, row: usize, col: usize, tile: u8) {
        self.board[row][col] = tile;
    }

    pub fn match_tile(&self, row: usize, col: usize, tile: u8) -> bool {
        if row < 0 || col < 0 {
            return true;
        }
        // println!("row:{}, col:{}", row, col);
        if row < BOARD_SIZE && col < BOARD_SIZE {
            // println!("BoardTile:{}", Tiles::number_to_tile(self.board[row][col]))
        }
        tile == 0 || (row < BOARD_SIZE && col < BOARD_SIZE && self.board[row][col] == tile)
    }

    /*
     *   implementation to check if the building can be placed on board
     */
    // todo : this should return a list of tuples of each possible coordinate... eventually
    pub fn match_building(&self, building: &Building) -> bool {
        /*
         * - outer loop rotate
         *     - inner loop vertical
         *         - inner-inner loop horizontal
         *             - check if building exists
         */
        for start_row in 0..BOARD_SIZE {
            if start_row >= BOARD_SIZE {
                break;
            }
            for start_col in 0..BOARD_SIZE {
                if start_col >= BOARD_SIZE {
                    break;
                }
                // check if building exists here
                if self.verify_building_rotation(&building, start_row, start_col) {
                    println!("YUPPPP!");
                    return true;
                }
            }
        }
        return false;
    }

    fn verify_building_rotation(
        &self,
        building: &Building,
        start_row: usize,
        start_col: usize,
    ) -> bool {
        println!("start_row: {}, start_col: {}", start_row, start_col);
        for rotation in 0..4 {
            match rotation {
                0 => {
                    // println!("start 0 degrees");
                    if self.verify_building(building, start_row, start_col) {
                        // println!("0 degrees");
                        return true;
                    }
                }
                1 => {
                    // println!("start 90 degrees");
                    if self.verify_building_90_degrees(building, start_row, start_col) {
                        // println!("90 degrees");
                        return true;
                    }
                }
                2 => {
                    // println!("start 180 degrees");
                    if self.verify_building_180_degrees(building, start_row, start_col) {
                        // println!("180 degrees");
                        return true;
                    }
                }
                3 => {
                    println!("start 270 degrees");
                    if self.verify_building_270_degrees(building, start_row, start_col) {
                        // println!("270 degrees");
                        return true;
                    }
                }
                _ => unreachable!(),
            };
        }
        return false;
    }

    /*
     *
     *  Search for building on board at start_row and start_col
     *  in this orientation:
     *
     *  | a, b, c, d|
     *  | e, f, g, h|
     *
     * Parameters:
     * building: building being searched for on board
     * start_row: start row for search
     * start_col: start col for search
     *
     */
    fn verify_building(&self, building: &Building, start_row: usize, start_col: usize) -> bool {
        for row in 0..building.get_shape().len() {
            for col in 0..building.get_shape()[0].len() {
                if !self.match_tile(
                    start_row + row,
                    start_col + col,
                    building.get_shape()[row][col],
                ) {
                    return false;
                }
            }
        }
        // add building to tiles
        true
    }

    /*
     *
     *  Search for building on board at start_row and start_col
     *  in this orientation (90 degrees clockwise):
     *
     *  | e, a|
     *  | f, b|
     *  | g, c|
     *  | h, d|
     *
     * Parameters:
     * building: building being searched for on board
     * start_row: start row for search
     * start_col: start col for search
     *
     */
    fn verify_building_90_degrees(
        &self,
        building: &Building,
        start_row: usize,
        start_col: usize,
    ) -> bool {
        for col in 0..building.get_shape()[0].len() {
            for row in (0..building.get_shape().len()).rev() {
                if !self.match_tile(
                    start_row + col,
                    start_col + (building.get_shape().len() - row - 1),
                    building.get_shape()[row][col],
                ) {
                    return false;
                }
            }
        }
        true
    }

    /*
     *
     *  Search for building on board at start_row and start_col
     *  in this orientation (180 degrees clockwise):
     *
     *  | h, g, f, e|
     *  | d, c, b, a|
     *
     * Parameters:
     * building: building being searched for on board
     * start_row: start row for search
     * start_col: start col for search
     *
     */
    fn verify_building_180_degrees(
        &self,
        building: &Building,
        start_row: usize,
        start_col: usize,
    ) -> bool {
        for row in (0..building.get_shape().len()).rev() {
            for col in (0..building.get_shape()[0].len()).rev() {
                // todo: move this check into match_tile function
                // There will be a false positive case if non-empty tiles are out of bounds
                let tile = building.get_shape()[row][col];
                if (Player::is_negative_index(start_col, col)
                    || Player::is_negative_index(start_row, row))
                {
                    if tile != 0 {
                        return false;
                    }
                    continue;
                }
                if !self.match_tile(start_row - row, start_col - col, tile) {
                    return false;
                }
            }
        }
        true
    }

    /*
     *
     *  Search for building on board at start_row and start_col
     *  in this orientation (270 degrees clockwise):
     *
     *  | d, h|
     *  | c, g|
     *  | b, f|
     *  | a, e|
     *
     * Parameters:
     * building: building being searched for on board
     * start_row: start row for search
     * start_col: start col for search
     *
     */
    fn verify_building_270_degrees(
        &self,
        building: &Building,
        start_row: usize,
        start_col: usize,
    ) -> bool {
        for col in (0..building.get_shape()[0].len()).rev() {
            for row in 0..building.get_shape().len() {
                // if start_row == 1 && start_col == 1 {
                //     if row < BOARD_SIZE && col < BOARD_SIZE {
                //         print!(
                //             "HEREDOOD: {}",
                //             Tiles::number_to_tile(building.get_shape()[row][col])
                //         )
                //     }
                // }
                let tile = building.get_shape()[row][col];
                let diff = start_row as i8 - col as i8;
                if Player::is_negative_index(start_row, col) {
                    if tile != 0 {
                        return false;
                    }
                    continue;
                }
                if !self.match_tile(start_row - col, start_col + row, tile) {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_negative_index(a: usize, b: usize) -> bool {
        return (a as i8 - b as i8) < 0;
    }
}
