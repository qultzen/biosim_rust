use std::collections::HashMap;

mod island {
    pub struct Parameters {
        pub allowed_cells: [char; 4],
    }

    pub const ISLAND: Parameters = Parameters {
        allowed_cells: ['W', 'H', 'L', 'D'],
    };
}

use island::ISLAND;

pub struct Island {
    raw_str: String,
    height: u32,
    width: u32,

    pop_in_cell: HashMap<String, HashMap<(u32, u32), u32>>,
    pop: HashMap<String, u32>,
}

impl Island {
    pub fn build(raw_str: &str) -> Result<Island, &'static str> {
        let pop_in_cell: HashMap<String, HashMap<(u32, u32), u32>> = HashMap::new();
        let pop: HashMap<String, u32> = HashMap::new();
        Ok(Island {
            raw_str: raw_str.to_string(),
            height: 1,
            width: 1,
            pop_in_cell,
            pop,
        })
    }

    pub fn process_map(&self) -> Vec<&str> {
        let mut lines = self.raw_str.lines().filter(|line| !line.trim().is_empty());

        let line_len = lines.next().map_or(0, |line| line.trim().len());

        self.raw_str
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                let line = line.trim();
                assert_eq!(line.len(), line_len, "Lines are not the same length");
                assert!(
                    line.chars().all(|c| ISLAND.allowed_cells.contains(&c)),
                    "Invalid charracter in line"
                );
                line
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
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

        let processed_map = island.process_map();

        assert_eq!(correct, processed_map);
    }

    #[test]
    #[should_panic]
    fn test_wrong_char() {
        let input_str = "
        WWW
        WEW
        WLW
        WWW";

        let island = Island::build(input_str).unwrap();
        let _ = island.process_map();
    }

    #[test]
    #[should_panic]
    fn test_wrong_len() {
        let input_str = "
        WWWW
        WEW
        WLW
        WWW";

        let island = Island::build(input_str).unwrap();
        let _ = island.process_map();
    }
}
