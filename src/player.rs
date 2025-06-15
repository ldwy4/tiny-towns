use std::collections::btree_map::Range;
use std::iter::Rev;

use crate::tiles::Tile;
use crate::tiles::building::BuildingType;
use crate::tiles::resources::Resources;

const BOARD_SIZE: usize = 4;
// TODO: change be a variable passed in on game creation
const BUILDING_TYPE_IN_GAME: usize = 7;

pub struct Player {
    board: [[Tile; BOARD_SIZE]; BOARD_SIZE],
    score: i32,
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
// pub struct PlayerTileInfo {
//     tile: TileType,
//     possible_buildings: [BuildingType; BUILDING_TYPE_IN_GAME],
// }

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
            board: [[Tile::Resource(Resources::Empty); BOARD_SIZE]; BOARD_SIZE],
            score: 0,
        }
    }

    pub fn print_board(&self) {
        // implementation to print the board
        println!("Board:");
        for row in &self.board {
            for cell in row {
                print!("{},", *cell);
            }
            print!("\n");
        }
    }

    pub fn place_tile(&mut self, row: usize, col: usize, tile: Tile) {
        self.board[row][col] = tile;
    }

    pub fn get_board(&self) -> &[[Tile; BOARD_SIZE]; BOARD_SIZE] {
        &self.board
    }
}
