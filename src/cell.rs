use crate::animal::{self, AnimalTrait, Carnivore, Herbivore, Species};
use rand::prelude::SliceRandom;
#[derive(Clone, Debug, PartialEq)]
pub struct Fauna {
    pub herbivore: Vec<Herbivore>,
    pub carnivore: Vec<Carnivore>,
}

impl Fauna {
    pub const fn new() -> Fauna {
        let herbivore = Vec::<Herbivore>::new();
        let carnivore = Vec::<Carnivore>::new();
        Fauna {
            herbivore,
            carnivore,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CellType {
    Water,
    Desert,
    Lowland,
    Highland,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Cell {
    name: CellType,
    pub loc: (u32, u32),
    pub fauna: Option<Fauna>,
    pub fodder: f32,
    f_max: f32,
}

impl<'a> Cell {
    pub fn get_cell(&self) -> CellType {
        self.name.clone()
    }

    pub fn add_herb_struct(&mut self, animal: Herbivore) {
        self.fauna.as_mut().unwrap().herbivore.push(animal);
    }

    pub fn add_carn_struct(&mut self, animal: Carnivore) {
        self.fauna.as_mut().unwrap().carnivore.push(animal);
    }
    // example of vec: vec![((1, 1), "Herbivore".to_string(), 200)]
    pub fn add_animal_from(&'a mut self, species: String, amount: u32) {
        match species.as_str() {
            "Herbivore" => {
                let herbivore = &mut self.fauna.as_mut().unwrap().herbivore;
                for _ in 0..amount {
                    let herb = animal::Herbivore::new();
                    herbivore.push(herb);
                }
            }
            "Carnivore" => {
                let carnivore = &mut self.fauna.as_mut().unwrap().carnivore;
                for _ in 0..amount {
                    let carn = animal::Carnivore::new();
                    carnivore.push(carn);
                }
            }
            name => panic!("Wrong species name: {name}"),
        }
    }

    // sort
    pub fn sort_herbivore_after_fitness(&mut self, descending: bool) {
        self.fauna.as_mut().unwrap().herbivore.sort_by(|a, b| {
            if descending {
                b.stats.fitness.partial_cmp(&a.stats.fitness).unwrap()
            } else {
                a.stats.fitness.partial_cmp(&b.stats.fitness).unwrap()
            }
        });
    }

    // feed animals
    pub fn feed_animals(&mut self) {
        if !self.fauna.as_ref().unwrap().herbivore.is_empty() {
            self.sort_herbivore_after_fitness(false);
            let fauna = &mut self.fauna.as_mut().unwrap();
            let herbivores = &mut fauna.herbivore;

            for herb in herbivores {
                if self.fodder > 0.0 {
                    self.fodder -= herb.feeding(self.fodder);
                } else {
                    break;
                }
            }
        }

        if !self.fauna.as_ref().unwrap().carnivore.is_empty() {
            self.sort_herbivore_after_fitness(false);

            let fauna = &mut self.fauna.as_mut().unwrap();

            let herbivores = &mut fauna.herbivore;

            let carnivores = &mut fauna.carnivore;
            // shuffle carnivores
            carnivores.shuffle(&mut rand::thread_rng());

            for carnivore in carnivores {
                carnivore.feeding(herbivores);

                // remove dead herbivores
                herbivores.retain(|herb| herb.stats.alive);
            }
        }
    }

    pub fn reset_fodder(&mut self) {
        self.fodder = self.f_max;
    }

    pub fn animal_death(&mut self) {
        let fauna = &mut self.fauna.as_mut().unwrap();
        let herbivores = &mut fauna.herbivore;
        let carnivores = &mut fauna.carnivore;

        herbivores.retain(|herb| herb.stats.alive);
        carnivores.retain(|carn| carn.stats.alive);
    }

    pub fn age_animals(&mut self) {
        let fauna = &mut self.fauna.as_mut().unwrap();
        let herbivores = &mut fauna.herbivore;
        let carnivores = &mut fauna.carnivore;

        // age herbivores in a fucntional way
        herbivores.iter_mut().for_each(|herb| herb.aging());
        carnivores.iter_mut().for_each(|carn| carn.aging());
    }

    pub fn loss_of_weight(&mut self) {
        let fauna = &mut self.fauna.as_mut().unwrap();
        let herbivores = &mut fauna.herbivore;
        let carnivores = &mut fauna.carnivore;

        herbivores.iter_mut().for_each(|herb| herb.loss_of_weight());
        carnivores.iter_mut().for_each(|carn| carn.loss_of_weight());
    }

    pub fn get_random_neighboring_cell(&self) -> Option<(u32, u32)> {
        let loc = self.loc.clone();
        let mut rng = rand::thread_rng();

        // move north, east, south, west
        let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

        let direction = directions.choose(&mut rng).unwrap();

        let x = loc.0 as i32 + direction.0;
        let y = loc.1 as i32 + direction.1;

        if x < 0 || y < 0 {
            return None;
        }

        Some((x as u32, y as u32))
    }

    //pub fn get_moving_animals(&mut self);
}

pub fn water(loc: (u32, u32)) -> Cell {
    Cell {
        name: CellType::Water,
        loc: (0, 0),
        fauna: None,
        fodder: 0.0,
        f_max: 0.0,
    }
}

pub fn desert(loc: (u32, u32)) -> Cell {
    Cell {
        name: CellType::Desert,
        fauna: Some(Fauna::new()),
        loc: (0, 0),
        fodder: 0.0,
        f_max: 0.0,
    }
}

pub fn lowland(loc: (u32, u32)) -> Cell {
    Cell {
        name: CellType::Lowland,
        fauna: Some(Fauna::new()),
        loc: (0, 0),
        fodder: 800.0,
        f_max: 800.0,
    }
}

pub fn highland(loc: (u32, u32)) -> Cell {
    Cell {
        name: CellType::Highland,
        fauna: Some(Fauna::new()),
        loc: (0, 0),
        fodder: 300.0,
        f_max: 300.0,
    }
}

pub fn from_char(c: char) -> Cell {
    match c {
        'W' => Cell {
            name: CellType::Water,
            loc: (0, 0),
            fauna: None,
            fodder: 0.0,
            f_max: 0.0,
        },
        'D' => Cell {
            name: CellType::Desert,
            fauna: Some(Fauna::new()),
            loc: (0, 0),
            fodder: 0.0,
            f_max: 0.0,
        },
        'L' => Cell {
            name: CellType::Lowland,
            loc: (0, 0),
            fauna: Some(Fauna::new()),
            fodder: 800.0,
            f_max: 800.0,
        },
        'H' => Cell {
            name: CellType::Highland,
            loc: (0, 0),
            fauna: Some(Fauna::new()),
            fodder: 300.0,
            f_max: 300.0,
        },
        _ => panic!(),
    }
}

#[cfg(test)]
mod test_cell_creation {
    use super::*;

    #[test]
    fn test_herb_struct() {
        let mut cell = lowland((1, 1));
        cell.add_carn_struct(animal::Carnivore::new());
        cell.add_carn_struct(animal::Carnivore::new());
        cell.add_herb_struct(animal::Herbivore::new());

        println!("{:#?}", cell.fauna);

        assert_eq!(cell.fauna.as_ref().unwrap().herbivore.len(), 1);
        assert_eq!(cell.fauna.as_ref().unwrap().carnivore.len(), 2);
    }

    #[test]
    fn test_water() {
        let cell = water((1, 1));
        assert_eq!(cell.get_cell(), CellType::Water);
    }

    #[test]
    fn test_desert() {
        let cell = desert((1, 1));
        assert_eq!(cell.get_cell(), CellType::Desert);
    }

    #[test]
    fn test_lowland() {
        let cell = lowland((1, 1));
        assert_eq!(cell.get_cell(), CellType::Lowland);
    }

    #[test]
    fn test_highland() {
        let cell = highland((1, 1));
        assert_eq!(cell.get_cell(), CellType::Highland);
    }

    #[test]
    fn test_from_char() {
        let cell = from_char('W');
        assert_eq!(cell.get_cell(), CellType::Water);
    }
}

#[cfg(test)]
mod test_cell_methods {
    use super::*;

    // test sort_herbivore_after_fitness
    #[test]
    fn test_sort_herbivore_after_fitness() {
        let mut cell = lowland((1, 1));
        let herb1 = Herbivore::new();
        let herb2 = Herbivore::new();
        let herb3 = Herbivore::new();
        let herb4 = Herbivore::new();
        let herb5 = Herbivore::new();
        let herb6 = Herbivore::new();
        let herb7 = Herbivore::new();
        let herb8 = Herbivore::new();
        let herb9 = Herbivore::new();
        let herb10 = Herbivore::new();

        let mut herb_vec = vec![
            herb1, herb2, herb3, herb4, herb5, herb6, herb7, herb8, herb9, herb10,
        ];

        for (i, herb) in herb_vec.iter_mut().enumerate() {
            herb.stats.fitness = i as f32;
        }

        cell.fauna.as_mut().unwrap().herbivore = herb_vec;

        println!("{:#?}", cell.fauna.as_ref().unwrap().herbivore);

        cell.sort_herbivore_after_fitness(true);

        println!("--------------------------------------------------------");
        println!("{:#?}", cell.fauna.as_ref().unwrap().herbivore);

        let sorted_herb = &cell.fauna.as_ref().unwrap().herbivore;

        for i in 0..sorted_herb.len() - 1 {
            assert!(
                sorted_herb[i].stats.fitness >= sorted_herb[i + 1].stats.fitness,
                "List not sorted in descending order"
            );
        }
    }

    // test feed herbivores
    #[test]
    fn test_feed_herbivores() {
        let mut cell = lowland((1, 1));
        let herb1 = Herbivore::new();
        let herb2 = Herbivore::new();
        let herb3 = Herbivore::new();
        let herb4 = Herbivore::new();
        let herb5 = Herbivore::new();
        let herb6 = Herbivore::new();
        let herb7 = Herbivore::new();
        let herb8 = Herbivore::new();
        let herb9 = Herbivore::new();
        let herb10 = Herbivore::new();

        let herb_vec = vec![
            herb1, herb2, herb3, herb4, herb5, herb6, herb7, herb8, herb9, herb10,
        ];

        cell.fauna.as_mut().unwrap().herbivore = herb_vec;

        cell.feed_animals();

        assert_ne!(cell.fodder, 800.0);
    }

    // test feed carnivores
    #[test]
    fn test_feed_carnivores() {
        let mut cell = lowland((1, 1));
        let herb1 = Herbivore::new();
        let herb2 = Herbivore::new();
        let herb3 = Herbivore::new();
        let herb4 = Herbivore::new();
        let herb5 = Herbivore::new();
        let herb6 = Herbivore::new();
        let herb7 = Herbivore::new();
        let herb8 = Herbivore::new();
        let herb9 = Herbivore::new();
        let herb10 = Herbivore::new();

        let herb_vec = vec![
            herb1, herb2, herb3, herb4, herb5, herb6, herb7, herb8, herb9, herb10,
        ];

        let carn1 = Carnivore::new();
        let carn2 = Carnivore::new();
        let carn3 = Carnivore::new();
        let carn4 = Carnivore::new();
        let carn5 = Carnivore::new();
        let carn6 = Carnivore::new();
        let carn7 = Carnivore::new();
        let carn8 = Carnivore::new();
        let carn9 = Carnivore::new();
        let carn10 = Carnivore::new();

        let carn_vec = vec![
            carn1, carn2, carn3, carn4, carn5, carn6, carn7, carn8, carn9, carn10,
        ];

        cell.fauna.as_mut().unwrap().herbivore = herb_vec;
        cell.fauna.as_mut().unwrap().carnivore = carn_vec;

        cell.feed_animals();

        println!(
            "len of hebivores: {:#?}",
            cell.fauna.as_ref().unwrap().herbivore.len()
        );

        println!("{:#?}", cell.fauna.as_ref().unwrap().carnivore);

        assert!(
            cell.fauna.as_ref().unwrap().herbivore.len() < 10,
            "No herb died"
        );
    }

    // test reset fodder
    #[test]
    fn test_reset_fodder() {
        let mut cell = lowland((1, 1));
        cell.fodder = 0.0;
        cell.reset_fodder();

        assert_eq!(cell.fodder, 800.0);
    }
}
