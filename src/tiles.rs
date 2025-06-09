use std::fmt;

pub enum Tiles {
    Empty,
    Wheat,
    Brick,
    Glass,
    Stone,
    Wood,
}

impl Tiles {
    pub fn number_to_tile(number: u8) -> Tiles {
        match number {
            0 => Tiles::Empty,
            1 => Tiles::Wheat,
            2 => Tiles::Brick,
            3 => Tiles::Glass,
            4 => Tiles::Stone,
            6 => Tiles::Wood,
            _ => Tiles::Empty,
        }
    }

    pub fn tile_to_number(tile: Tiles) -> u8 {
        match tile {
            Tiles::Empty => 0,
            Tiles::Wheat => 1,
            Tiles::Brick => 2,
            Tiles::Glass => 3,
            Tiles::Stone => 4,
            Tiles::Wood => 6,
        }
    }

    pub fn string_to_tile(tile: &str) -> Tiles {
        match tile {
            "Wheat" | "wheat" => Tiles::Wheat,
            "Brick" | "brick" => Tiles::Brick,
            "Glass" | "glass" => Tiles::Glass,
            "Stone" | "stone" => Tiles::Stone,
            "Wood" | "wood" => Tiles::Wood,
            _ => Tiles::Empty,
        }
    }
}

impl fmt::Display for Tiles {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Tiles::Wheat => String::from("Wheat"),
            Tiles::Brick => String::from("Brick"),
            Tiles::Glass => String::from("Glass"),
            Tiles::Stone => String::from("Stone"),
            Tiles::Wood => String::from("Wood"),
            _ => String::from("Empty"),
        };
        write!(f, "{}", s)
    }
}
