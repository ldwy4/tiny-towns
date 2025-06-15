use std::fmt;

#[derive(PartialEq, Copy, Clone)]
pub enum Resources {
    Empty = 0,
    Wheat = 1,
    Brick = 2,
    Glass = 3,
    Stone = 4,
    Wood = 5,
}

impl Resources {
    pub fn number_to_tile(number: u8) -> Resources {
        match number {
            0 => Resources::Empty,
            1 => Resources::Wheat,
            2 => Resources::Brick,
            3 => Resources::Glass,
            4 => Resources::Stone,
            5 => Resources::Wood,
            _ => Resources::Empty,
        }
    }

    pub fn tile_to_number(tile: Resources) -> u8 {
        match tile {
            Resources::Empty => 0,
            Resources::Wheat => 1,
            Resources::Brick => 2,
            Resources::Glass => 3,
            Resources::Stone => 4,
            Resources::Wood => 5,
        }
    }

    pub fn string_to_tile(tile: &str) -> Resources {
        match tile {
            "Wheat" | "wheat" => Resources::Wheat,
            "Brick" | "brick" => Resources::Brick,
            "Glass" | "glass" => Resources::Glass,
            "Stone" | "stone" => Resources::Stone,
            "Wood" | "wood" => Resources::Wood,
            _ => Resources::Empty,
        }
    }
}

impl fmt::Display for Resources {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Resources::Wheat => String::from("Wheat"),
            Resources::Brick => String::from("Brick"),
            Resources::Glass => String::from("Glass"),
            Resources::Stone => String::from("Stone"),
            Resources::Wood => String::from("Wood"),
            _ => String::from("Empty"),
        };
        write!(f, "{}", s)
    }
}
