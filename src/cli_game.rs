use crate::placement_logic::building_matcher::match_building;
use crate::placement_logic::building_placer::place_building;
use crate::player::{self, Player};
use crate::tiles::Tile;
use crate::tiles::building::{ALL_BUILDINGS, Building, BuildingType};
use crate::tiles::resources::Resources;
use std::io;

pub fn game_initiate() {
    let mut player: Player = Player::new();
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
    select_tile_loop(&mut player, &building);
}

fn select_tile_loop(player: &mut Player, building: &Building) {
    let mut input = String::new();

    loop {
        println!("Wanna try placing a building?");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim() == "exit" {
            break;
        }

        if input.trim() == "yes" {
            input.clear();
            println!("Which building?");
            for i in ALL_BUILDINGS {
                println!("{}", i);
            }

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let buildingType = BuildingType::string_to_building(&input.trim());
            input.clear();

            println!("Which row (0-3)?");

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let row = input.trim().parse().expect("Enter number between 0-3");
            input.clear();

            println!("Which col (0-3)?");

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let col = input.trim().parse().expect("Enter number between 0-3");
            input.clear();

            println!("Which rotation (0-3)?");

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let rotation = input.trim().parse().expect("Enter number between 0-3");
            input.clear();

            place_building(player, &building, row, col, rotation);
            player.print_board();
        }
        input.clear();
        println!("Enter a tile type:");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim() == "exit" {
            break;
        }

        let tile = Resources::tile_to_number(Resources::string_to_tile(&input.trim()));
        input.clear();

        if tile != Resources::tile_to_number(Resources::Empty) {
            place_tile_loop(player, tile)
        }
        player.print_board();
        if match_building(player, building) {
            println!("YOU CAN BUILD A HOUSE")
        }
    }
}

pub fn place_tile_loop(player: &mut Player, tile: u8) {
    let mut row: usize;
    let mut col: usize;

    let mut input = String::new();
    loop {
        println!("Enter row (0-3):");

        // read input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        row = input.trim().parse().expect("Enter number between 0-3");
        input.clear(); // clear buffer

        match row {
            0..4 => break,
            _ => println!("Enter number between 0 and 3:"),
        }
    }

    loop {
        println!("Enter col (0-3):");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        col = input.trim().parse().expect("Enter number between 0-3");
        input.clear(); // clear buffer
        match col {
            0..4 => break,
            _ => println!("Enter number between 0 and 3:"),
        }
    }

    player.place_tile(row, col, Tile::Resource(Resources::number_to_tile(tile)));
}
