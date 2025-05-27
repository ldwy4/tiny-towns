use std::collections::btree_map::Range;
use std::iter::Rev;

use crate::building::Building;
use crate::tiles::Tiles;
pub struct Player {
    board: [[u8; 4]; 4],
    score: i32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            board: [[0; 4]; 4],
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
        if row < 4 && col < 4 {
            println!("BoardTile:{}", Tiles::number_to_tile(self.board[row][col]))
        }
        tile == 0 || (row < 4 && col < 4 && self.board[row][col] == tile)
    }

    /*
     *   implementation to check if the building can be placed on board
     */
    // todo : this should return a list of tuples of each possible coordinate
    pub fn match_building(&self, building: Building) -> bool {
        /*
         * - outer loop rotate
         *     - inner loop vertical
         *         - inner-inner loop horizontal
         *             - check if building exists
         */
        let height =
            building.last_non_empty_tile_col() + if building.get_second_floor() { 1 } else { 0 };
        for start_row in 0..self.board.len() {
            if start_row + building.last_non_empty_tile_row() > self.board.len() {
                break;
            }
            for start_col in 0..self.board[0].len() {
                if start_col + building.last_non_empty_tile_col() > self.board[0].len() {
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
        for rotation in 0..4 {
            match rotation {
                0 => {
                    println!("start 0 degrees");
                    if self.verify_building(building, start_row, start_col) {
                        println!("0 degrees");
                        return true;
                    }
                }
                1 => {
                    println!("start 90 degrees");
                    if self.verify_building_90_degrees(building, start_row, start_col) {
                        println!("90 degrees");
                        return true;
                    }
                }
                2 => {
                    println!("start 180 degrees");
                    if self.verify_building_180_degrees(building, start_row, start_col) {
                        println!("180 degrees");
                        return true;
                    }
                }
                3 => {
                    println!("start 270 degrees");
                    if self.verify_building_270_degrees(building, start_row, start_col) {
                        println!("270 degrees");
                        return true;
                    }
                }
                _ => unreachable!(),
            };
        }
        return false;
    }

    fn verify_building(&self, building: &Building, start_row: usize, start_col: usize) -> bool {
        println!(
            "start_row: {}, start_col: {}",
            start_row,
            start_col
        );
        for row in 0..building.get_shape().len() {
            for col in 0..building.get_shape()[0].len() {
                print!("{},", Tiles::number_to_tile(building.get_shape()[row][col]));
                if !self.match_tile(start_row+row, start_col+col, building.get_shape()[row][col]) {
                    println!("NOPE! Building cannot be placed on board");
                    return false;
                }
                print!("\n");
            }
            print!("\n");
        }
        true
    }

    fn verify_building_90_degrees(&self, building: &Building, start_row: usize, start_col: usize) -> bool {
        for col in 0..building.get_shape()[0].len() {
            for row in (0..building.get_shape().len()).rev() {
                println!(
                    "row: {}, col: {}",
                    start_col+(building.get_shape().len()-row), start_row+col
                );
                print!("Building: {},", Tiles::number_to_tile(building.get_shape()[row][col]));
                if !self.match_tile(start_row+col, start_col+(building.get_shape().len()-row-1), building.get_shape()[row][col]) {
                    println!("NOPE! Building cannot be placed on board");
                    return false;
                }
                print!("\n");
            }
            print!("\n");
        }
        true
    }

    fn verify_building_180_degrees(&self, building: &Building, start_row: usize, start_col: usize) -> bool {
        for row in (0..building.get_shape().len()).rev() {
            for col in (0..building.get_shape()[0].len()).rev() {
                print!("{},", Tiles::number_to_tile(building.get_shape()[row][col]));
                if !self.match_tile(start_row + (building.get_shape().len()-row-1),
                 start_col + (building.get_shape()[0].len()-col-1),
                building.get_shape()[row][col]) {
                    println!("NOPE! Building cannot be placed on board");
                    return false;
                }
            }
            print!("\n");
        }
        true
    }

    fn verify_building_270_degrees(&self, building: &Building, start_row: usize, start_col: usize) -> bool {
        for col in (0..building.get_shape()[0].len()).rev() {
            for row in 0..building.get_shape().len() {
                print!("{},", Tiles::number_to_tile(building.get_shape()[row][col]));
                if !self.match_tile(start_col + row, start_row + col, building.get_shape()[row][col]) {
                    println!("NOPE! Building cannot be placed on board");
                    return false;
                }
            }
            print!("\n");
        }
        true
    }
}
