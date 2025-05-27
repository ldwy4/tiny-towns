mod building;
mod player;
mod tiles;
use building::Building;
use building::BuildingType;
use player::Player;
use tiles::Tiles;

fn main() {
    // let player = Player::new();
    let tile = Tiles::Glass.to_string();
    println!("Tile: {}", tile);
    let mut player: Player = Player::new();
    player.place_tile(1, 1, 1);
    player.place_tile(2, 1, 2);
    player.place_tile(3, 1, 3);
    player.place_tile(1, 0, 4);
    player.place_tile(0, 2, 2);
    player.place_tile(0, 1, 3);

    player.print_board();

    let building = Building::new(BuildingType::House, [[1, 2, 3, 0], [0, 0, 0, 0]], false);
    Building::print_building(&building);

    player.match_building(building);
}
