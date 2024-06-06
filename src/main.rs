use biosim_rust::Config;
use std::process;

pub mod animal;
pub mod cell;
pub mod island;
pub mod simulation;

use island::Island;

fn main() {
    let input_map = "
        WWWW
        WLLW
        WLLW
        WWWW"
        .to_string();

    let input_pop = vec![
        ((1, 1), "Herbivore".to_string(), 80),
        ((1, 1), "Carnivore".to_string(), 1),
    ];

    let mut island = Island::build(&input_map).unwrap();

    for (coord, species, amount) in input_pop {
        island
            .map
            .get_mut(&coord)
            .unwrap()
            .add_animal_from(species, amount)
    }

    let mut herb;
    let mut carn;
    for i in 0..100 {
        (herb, carn) = island.get_pop();
        println!("{i}: {herb}, {carn}");
        island.yearly_cycle();
    }
}
