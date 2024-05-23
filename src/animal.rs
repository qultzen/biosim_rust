mod animals_params {
    #[derive(PartialEq, Debug, Clone)]
    pub enum Species {
        Herbivore,
        Carnivore,
    }

    #[derive(PartialEq, Debug)]
    pub struct Parameters {
        pub w_birth: f32,
        pub mu: f32,
        pub sigma_birth: f32,
        pub beta: f32,
        pub eta: f32,
        pub a_half: f32,
        pub phi_age: f32,
        pub w_half: f32,
        pub phi_weight: f32,
        pub gamma: f32,
        pub zeta: f32,
        pub xi: f32,
        pub omega: f32,
        pub f: f32,
        pub delta_phi_max: f32,
    }

    pub const HERBIVORE: Parameters = Parameters {
        w_birth: 8.0,
        mu: 0.25,
        sigma_birth: 1.5,
        beta: 0.9,
        eta: 0.05,
        a_half: 40.0,
        phi_age: 0.6,
        w_half: 10.0,
        phi_weight: 0.1,
        gamma: 0.2,
        zeta: 3.5,
        xi: 1.2,
        omega: 0.4,
        f: 10.0,
        delta_phi_max: 0.0,
    };

    pub const CARNIVORE: Parameters = Parameters {
        w_birth: 6.0,
        mu: 0.4,
        sigma_birth: 1.0,
        beta: 0.75,
        eta: 0.125,
        a_half: 40.0,
        phi_age: 0.3,
        w_half: 4.0,
        phi_weight: 0.4,
        gamma: 0.8,
        zeta: 3.5,
        xi: 1.1,
        omega: 0.8,
        f: 50.0,
        delta_phi_max: 10.0,
    };

    #[derive(PartialEq, Debug, Clone)]
    pub struct Stats {
        pub age: u32,
        pub weight: f32,
        pub fitness: f32,
        pub alive: bool,
        pub move_to: Option<((u32, u32))>,
    }

    impl Stats {
        pub fn new_default() -> Stats {
            Stats {
                age: 5,
                weight: 20.0,
                fitness: 0.0,
                alive: true,
                move_to: None,
            }
        }

        pub fn from(age: u32, weight: f32) -> Stats {
            Stats {
                age,
                weight,
                fitness: 0.0,
                alive: true,
                move_to: None,
            }
        }
    }
}

pub use animals_params::{Parameters, Species, Stats, CARNIVORE, HERBIVORE};
use rand::Rng;
use rand_distr::{Distribution, LogNormal};

fn random() -> f32 {
    rand::thread_rng().gen::<f32>()
}

pub trait AnimalTrait {
    fn get_birthweight(&mut self, count_in_cell: usize) -> Option<f32> {
        let zeta = self.params().zeta;
        let w_birth = self.params().w_birth;
        let sigma_birth = self.params().sigma_birth;
        let gamma = self.params().gamma;
        let xi = self.params().xi;

        // Calcuate probability of procreation
        let offspring_value = zeta * (w_birth * sigma_birth);

        if self.stats_as_ref().weight < offspring_value {
            return None;
        }

        let probability_of_procreation = f32::min(
            1.0,
            gamma * self.stats_as_ref().fitness * count_in_cell as f32,
        );

        if random() > probability_of_procreation {
            return None;
        }

        let mu = f32::ln(w_birth.powi(2) / f32::sqrt(w_birth.powi(2) + sigma_birth.powi(2)));
        let sigma = f32::sqrt(f32::ln(1.0 + (sigma_birth.powi(2) / w_birth.powi(2))));

        let log_normal = LogNormal::new(mu, sigma).unwrap();
        let newborn_weight = log_normal.sample(&mut rand::thread_rng());

        // check if parent has enought weight to give birth
        let parent_loss = xi * newborn_weight;

        if self.stats_as_ref().weight < parent_loss {
            return None;
        }

        self.stats_as_mut().weight -= parent_loss;
        self.update_fitness();

        return Some(newborn_weight);
    }

    fn calc_fitness(&self) -> f32 {
        if self.stats_as_ref().weight <= 0.0 {
            return 0.0;
        }

        let phi_weight = self.params().phi_weight;
        let phi_age = self.params().phi_age;
        let a_half = self.params().a_half;
        let w_half = self.params().w_half;

        let age_parameter =
            1.0 / (1.0 + f32::exp(phi_age * (self.stats_as_ref().age as f32 - a_half)));
        let weight_parameter =
            1.0 / (1.0 + f32::exp(-phi_weight * (self.stats_as_ref().weight - w_half)));

        return age_parameter * weight_parameter;
    }

    fn update_fitness(&mut self) {
        self.stats_as_mut().fitness = self.calc_fitness();
    }

    fn aging(&mut self) {
        self.stats_as_mut().age += 1;
    }

    fn loss_of_weight(&mut self) {
        self.stats_as_mut().weight -= self.params().eta * self.stats_as_ref().weight;
        self.update_fitness();
    }

    fn death(&mut self) {
        if self.stats_as_ref().weight <= 0.0 {
            self.stats_as_mut().alive = false;
        }

        let probability_of_death = self.params().omega * (1.0 - self.stats_as_ref().fitness);

        if random() < probability_of_death {
            self.stats_as_mut().alive = false;
        }
    }

    fn migrate(&self) -> bool {
        let probability_of_migration = self.params().mu * self.stats_as_ref().fitness;

        if random() < probability_of_migration {
            return true;
        } else {
            return false;
        }
    }

    fn species(&self) -> Species;

    fn stats_as_mut(&mut self) -> &mut Stats;

    fn stats_as_ref(&self) -> &Stats;

    fn params(&self) -> &Parameters;
}

#[derive(PartialEq, Debug, Clone)]
pub struct Herbivore {
    pub species: Species,
    pub stats: Stats,
}

impl AnimalTrait for Herbivore {
    fn stats_as_mut(&mut self) -> &mut Stats {
        &mut self.stats
    }

    fn stats_as_ref(&self) -> &Stats {
        &self.stats
    }

    fn params(&self) -> &Parameters {
        &HERBIVORE
    }

    fn species(&self) -> Species {
        self.species.clone()
    }
}

impl Herbivore {
    pub fn new() -> Herbivore {
        let stats = Stats::new_default();
        // update stats.fitness before init
        let mut herb = Herbivore {
            species: Species::Herbivore,
            stats: Stats::new_default(),
        };
        herb.update_fitness();
        herb
    }

    pub fn from(stats: Stats) -> Herbivore {
        let mut herb = Herbivore {
            species: Species::Herbivore,
            stats,
        };

        herb.update_fitness();
        herb
    }

    pub fn procreation(&mut self, count_in_cell: usize) -> Option<Self> {
        if let Some(newborn_weight) = self.get_birthweight(count_in_cell) {
            let stats = Stats::from(0, newborn_weight);
            return Some(Herbivore::from(stats));
        }

        None
    }

    pub fn feeding(&mut self, fodder: f32) -> f32 {
        let amount_eaten;
        if fodder < self.params().f {
            amount_eaten = fodder;
        } else {
            amount_eaten = self.params().f;
        }

        self.stats_as_mut().weight -= amount_eaten * self.params().beta;
        self.update_fitness();
        amount_eaten
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Carnivore {
    pub species: Species,
    pub stats: Stats,
}

impl AnimalTrait for Carnivore {
    fn stats_as_mut(&mut self) -> &mut Stats {
        &mut self.stats
    }

    fn stats_as_ref(&self) -> &Stats {
        &self.stats
    }

    fn params(&self) -> &Parameters {
        &CARNIVORE
    }

    fn species(&self) -> Species {
        self.species.clone()
    }
}

impl Carnivore {
    pub fn new() -> Carnivore {
        let mut carn = Carnivore {
            species: Species::Carnivore,
            stats: Stats::new_default(),
        };
        carn.update_fitness();
        carn
    }

    pub fn from(stats: Stats) -> Carnivore {
        let mut carn = Carnivore {
            species: Species::Carnivore,
            stats,
        };

        carn.update_fitness();
        carn
    }

    pub fn procreation(&mut self, count_in_cell: usize) -> Option<Self> {
        if let Some(newborn_weight) = self.get_birthweight(count_in_cell) {
            let stats = Stats::from(0, newborn_weight);
            return Some(Carnivore::from(stats));
        }

        None
    }

    pub fn feeding(&mut self, herb_sorted_lowest_fitness: &mut Vec<Herbivore>) {
        let delta_phi_max = self.params().delta_phi_max;
        let mut amount_eaten: f32 = 0.0;

        for herbivore in herb_sorted_lowest_fitness {
            if amount_eaten >= self.params().f {
                break;
            }

            let diff_fitness = self.stats_as_ref().fitness - herbivore.stats.fitness;

            if diff_fitness < 0.0 {
                continue;
            }

            let probability_of_killing;
            if 0.0 < diff_fitness && diff_fitness < delta_phi_max {
                probability_of_killing = (diff_fitness) / delta_phi_max;
            } else {
                probability_of_killing = 1.0;
            }

            if random() >= probability_of_killing {
                continue;
            }

            let desired_food = self.params().f - amount_eaten;
            let eating;

            if herbivore.stats.weight > desired_food {
                eating = desired_food;
            } else {
                eating = herbivore.stats.weight;
            }

            self.stats.weight += eating * self.params().beta;
            herbivore.stats.alive = false;
            self.update_fitness();
            amount_eaten += eating;
        }
    }
}

#[cfg(test)]
mod test_creation {
    use super::*;

    #[test]
    fn create_herb() {
        let expected = Herbivore::new();

        let mut result = Herbivore {
            species: Species::Herbivore,
            stats: Stats::new_default(),
        };
        result.update_fitness();

        assert_eq!(expected, result)
    }
    #[test]
    fn create_carn() {
        let expected = Carnivore::new();

        let mut result = Carnivore {
            species: Species::Carnivore,
            stats: Stats::new_default(),
        };
        result.update_fitness();

        assert_eq!(expected, result)
    }
}

#[cfg(test)]
mod test_attributes {
    // test the rest of the methods
    use super::*;

    #[test]
    fn test_age() {
        let mut herb = Herbivore::new();
        herb.aging();
        assert_eq!(herb.stats.age, 6);

        let mut carn = Carnivore::new();
        carn.aging();
        assert_eq!(carn.stats.age, 6);
    }

    #[test]
    fn test_loss_of_weight() {
        let mut herb = Herbivore::new();
        herb.stats.weight = 5.0;
        herb.loss_of_weight();

        // weight started as 5.0
        assert_eq!(herb.stats.weight, 4.75);

        let mut carn = Carnivore::new();
        carn.stats.weight = 5.0;
        carn.loss_of_weight();

        // weight started as 5.0
        assert_eq!(carn.stats.weight, 4.375);
    }

    #[test]
    fn feeding_animal() {
        let mut herb = Herbivore::new();
        herb.stats.weight = 10.0;
        herb.feeding(1.0);

        assert_eq!(herb.stats.weight, 9.1);

        herb.stats.fitness = 0.01;

        let mut herbs = vec![herb];

        let mut carn = Carnivore::new();
        carn.stats.weight = 20.0;
        carn.stats.fitness = 20.0;
        carn.feeding(&mut herbs);

        println!("{:#?}", herbs);

        assert_eq!(carn.stats.weight, 26.825);
    }
}
