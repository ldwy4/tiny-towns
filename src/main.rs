mod building;
mod building_matcher;
mod player;
mod tiles;
mod cli_game;
use building::Building;
use building::BuildingType;
use player::Player;
use tiles::Tiles;

fn main() {
    // let player = Player::new();
    let mut player: Player = Player::new();
    let building = Building::new(BuildingType::House,
         [[1, 2, 0, 0], [0, 0, 0, 0]], false);
    cli_game::select_tile_loops(&mut player);
    // let number: i32 = loop {
    //     println!("Enter a valid number:");

    //     let mut input = String::new();
    //     io::stdin().read_line(&mut input).unwrap();

    //     if Tiles.string_to_tile(input) != Tiles::Empty {
    //         player.place_tile(row, col, tile);
    //     }
    //     match input.trim().parse() {
    //         Ok(num) => break num,
    //         Err(_) => println!("Not a valid number, try again."),
    //     }
    // };
}
