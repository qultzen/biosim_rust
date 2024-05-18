use biosim_rust::Config;
use std::process;

fn main() {
    let input_map = "
        WWW
        WHW
        WWW"
    .to_string();

    let input_pop = vec![((1, 1), "Herbivore".to_string(), 200)];
    let config = Config::build(input_map.clone(), input_pop.clone()).unwrap();

    if let Err(e) = biosim_rust::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
