use std::fmt;

pub mod building;
pub mod resources;

pub use building::Building;
pub use resources::Resources;

pub use building::BuildingType;

#[derive(PartialEq, Copy, Clone)]
pub enum Tile {
    Building(BuildingType),
    Resource(Resources),
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Tile::Building(building_type) => write!(f, "Building({})", building_type),
            Tile::Resource(resources) => write!(f, "Resource({})", resources),
        }
    }
}

impl Tile {
    pub fn is_empty(&self) -> bool {
        match self {
            Tile::Resource(Resources::Empty) => true,
            _ => false,
        }
    }
}
