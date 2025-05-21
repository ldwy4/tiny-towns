use crate::tiles::Tiles;

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

    pub fn last_non_empty_tile_row(&self) -> usize {
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

    pub fn last_non_empty_tile_col(&self) -> usize {
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
