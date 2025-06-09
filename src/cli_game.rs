use crate::player::{self, Player};
use crate::tiles::Tiles;
use crate::building::{Building, BuildingType};

use std::io;

pub fn select_tile_loops(player: &mut Player) {
    let mut input = String::new();
    let building = Building::new(BuildingType::House,
        [[1, 2, 0, 0], [0, 0, 0, 0]], false);
    loop {
        println!("Enter a tile type:");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim() == "exit" {
            break;
        }

        let tile = Tiles::tile_to_number(Tiles::string_to_tile(&input.trim()));
        input.clear();

        if tile != Tiles::tile_to_number(Tiles::Empty) {
            place_tile_loop(player, tile)
        }
        player.print_board();
        if player.match_building(&building) {
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

    player.place_tile(row, col, tile);
}