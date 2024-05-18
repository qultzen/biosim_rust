use crate::cell::{Cell, DESERT, HIGHLAND, LOWLAND, WATER};
use std::{collections::HashMap, error::Error};

mod island_params {
    pub struct Parameters {
        pub allowed_cells: [char; 4],
    }

    pub const ISLAND: Parameters = Parameters {
        allowed_cells: ['W', 'H', 'L', 'D'],
    };
}

use island_params::ISLAND;

pub struct Island<'a> {
    raw_str: String,
    map_vec: Vec<&'a str>,
    height: usize,
    width: usize,

    map: HashMap<(u32, u32), Cell>,

    pop_in_cell: HashMap<String, HashMap<(u32, u32), u32>>,
    pop: HashMap<String, u32>,
}

impl Island<'_> {
    pub fn build(raw_str: &str) -> Result<Island, Box<dyn Error>> {
        let map_vec = Island::raw_map_to_vec(raw_str)?;
        let height = map_vec.len();
        let width = map_vec[0].len();
        let map = Island::vec_to_map(&map_vec);

        let pop_in_cell: HashMap<String, HashMap<(u32, u32), u32>> = HashMap::new();
        let pop: HashMap<String, u32> = HashMap::new();

        let island = Island {
            raw_str: raw_str.to_string(),
            map_vec,
            height,
            width,
            map,
            pop_in_cell,
            pop,
        };

        Ok(island)
    }

    pub fn map_vec(&self) -> &Vec<&str> {
        &self.map_vec
    }

    pub fn map(&self) -> &HashMap<(u32, u32), Cell> {
        &self.map
    }

    pub fn raw_map_to_vec(input_str: &str) -> Result<Vec<&str>, &'static str> {
        let mut lines = input_str.lines().filter(|line| !line.trim().is_empty());

        let line_len = lines.next().map_or(0, |line| line.trim().len());

        input_str
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                let line = line.trim();
                if line.len() != line_len {
                    return Err("Lines are not the same length");
                }
                if !line.chars().all(|c| ISLAND.allowed_cells.contains(&c)) {
                    return Err("Invalid character in line");
                }
                Ok(line)
            })
            .collect()
    }

    fn vec_to_map(map_vec: &Vec<&str>) -> HashMap<(u32, u32), Cell> {
        map_vec
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, cell)| ((x as u32, y as u32), crate::cell::from_char(cell)))
            })
            .collect()
    }
}

#[cfg(test)]
mod island_tests {
    use super::*;

    #[test]
    fn test_process() {
        let input_str = "
WWW
WHW
WLW
WWW";
        println!("{input_str}");
        let correct = vec!["WWW", "WHW", "WLW", "WWW"];

        let island = Island::build(input_str).unwrap();

        let map_vec = island.map_vec();

        assert_eq!(correct, map_vec.clone());
    }

    #[test]
    #[should_panic]
    fn test_wrong_char() {
        let input_str = "
        WWW
        WEW
        WLW
        WWW";

        let _ = Island::build(input_str).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_wrong_len() {
        let input_str = "
        WWWW
        WEW
        WLW
        WWW";

        let _ = Island::build(input_str).unwrap();
    }

    #[test]
    fn test_map_dict() {
        let input_str = "
        WWWWW
        WDHLW
        WWWWW";

        let island = Island::build(input_str).unwrap();

        let island_map = island.map().clone();

        let map_vec: HashMap<(u32, u32), Cell> = [
            ((0, 0), WATER),
            ((1, 0), WATER),
            ((2, 0), WATER),
            ((3, 0), WATER),
            ((4, 0), WATER),
            ((0, 1), WATER),
            ((1, 1), DESERT),
            ((2, 1), HIGHLAND),
            ((3, 1), LOWLAND),
            ((4, 1), WATER),
            ((0, 2), WATER),
            ((1, 2), WATER),
            ((2, 2), WATER),
            ((3, 2), WATER),
            ((4, 2), WATER),
        ]
        .iter()
        .cloned()
        .collect();

        let expected = HashMap::from(map_vec);

        assert_eq!(island_map, expected, "Hashmap of map is not equal");
    }
}
