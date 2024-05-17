use std::collections::HashMap;
mod island;
pub struct Config {
    island_map: String,
    ini_pop: Vec<((u32, u32), String, u32)>,
}

impl Config {
    pub fn build(island_map: String, ini_pop: Vec<((u32, u32), String, u32)>) -> Result<Config, &'static str> {
        Ok(Config{island_map, ini_pop})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_init() {
        let input_map = "WHW".to_string();
        let input_pop = vec![((1,1), "Herbivore".to_string(), 200)];
        let test_config = Config::build(input_map.clone(), input_pop.clone()).unwrap();
        let Config{island_map, ini_pop} = test_config; 

        assert_eq!((island_map, ini_pop), (input_map, input_pop));
    }
}
