use std::process;

mod lib;
use lib::Config;

fn main() {
    let input_map = "
    WWWW
    WEW
    WWW"
    .to_string();
    let input_pop = vec![((1, 1), "Herbivore".to_string(), 200)];
    let config = Config::build(input_map.clone(), input_pop.clone()).unwrap();

    if let Err(e) = lib::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
