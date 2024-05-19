mod animals_params {
    #[derive(PartialEq, Debug)]
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
        a_half: 4.0,
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

    #[derive(PartialEq, Debug)]
    pub struct Stats {
        pub age: u32,
        pub weight: f32,
        pub fitness: f32,
        pub alive: bool,
    }

    impl Stats {
        pub fn new_default() -> Stats {
            Stats {
                age: 5,
                weight: 20.0,
                fitness: 0.0,
                alive: true,
            }
        }

        pub fn from(age: u32, weight: f32) -> Stats {
            Stats {
                age,
                weight,
                fitness: 0.0,
                alive: true,
            }
        }
    }
}

use animals_params::{Parameters, Species, Stats, CARNIVORE, HERBIVORE};
use rand::Rng;
use rand_distr::{Distribution, LogNormal};

fn random() -> f32 {
    rand::thread_rng().gen::<f32>()
}

pub trait AnimalTrait {
    fn get_birthweight(&mut self, count_in_cell: u32) -> Option<f32> {
        let zeta = self.params().zeta;
        let w_birth = self.params().w_birth;
        let sigma_birth = self.params().sigma_birth;
        let gamma = self.params().gamma;
        let xi = self.params().xi;

        // Calcuate probability of procreation
        let offspring_value = zeta * (w_birth * sigma_birth);

        if self.stats().weight < offspring_value {
            return None;
        }

        let probability_of_procreation =
            f32::min(1.0, gamma * self.stats().fitness * count_in_cell as f32);

        if random() > probability_of_procreation {
            return None;
        }

        let mu = f32::ln(w_birth.powi(2) / f32::sqrt(w_birth.powi(2) + sigma_birth.powi(2)));
        let sigma = f32::sqrt(f32::ln(1.0 + (sigma_birth.powi(2) / w_birth.powi(2))));

        let log_normal = LogNormal::new(mu, sigma).unwrap();
        let newborn_weight = log_normal.sample(&mut rand::thread_rng());

        // check if parent has enought weight to give birth
        let parent_loss = xi * newborn_weight;

        if self.stats().weight < parent_loss {
            return None;
        }

        self.stats().weight -= parent_loss;
        self.update_fitness();

        return Some(newborn_weight);
    }

    fn calc_fitness(&mut self) -> f32 {
        if self.stats().weight <= 0.0 {
            return 0.0;
        }

        let phi_weight = self.params().phi_weight;
        let phi_age = self.params().phi_age;
        let a_half = self.params().a_half;
        let w_half = self.params().w_half;

        let age_parameter = 1.0 / (1.0 + (phi_age * (self.stats().age as f32 - a_half).exp()));
        let weight_parameter = 1.0 / (1.0 + (-phi_weight * (self.stats().weight - w_half).exp()));

        return age_parameter * weight_parameter;
    }

    fn update_fitness(&mut self) {
        self.stats().fitness = self.calc_fitness();
    }

    fn aging(&mut self) {
        self.stats().age += 1;
    }

    fn loss_of_weight(&mut self) {
        self.stats().weight -= self.params().eta * self.stats().weight;
        self.update_fitness();
    }

    fn death(&mut self) {
        if self.stats().weight <= 0.0 {
            self.stats().alive = false;
        }

        let probability_of_death = self.params().omega * (1.0 - self.stats().fitness);

        if random() < probability_of_death {
            self.stats().alive = false;
        }
    }

    fn migrate(&mut self) -> bool {
        let probability_of_migration = self.params().mu * self.stats().fitness;

        if random() < probability_of_migration {
            return true;
        } else {
            return false;
        }
    }

    fn stats(&mut self) -> &mut Stats;

    fn params(&self) -> &Parameters;
}

#[derive(PartialEq, Debug)]
pub struct Herbivore<'a> {
    species: Species,
    params: &'a Parameters,
    stats: Stats,
}

impl AnimalTrait for Herbivore<'_> {
    fn stats(&mut self) -> &mut Stats {
        &mut self.stats
    }

    fn params(&self) -> &Parameters {
        &self.params
    }
}

impl<'a> Herbivore<'a> {
    pub fn new() -> Herbivore<'a> {
        let stats = Stats::new_default();
        // update stats.fitness before init
        Herbivore {
            species: Species::Herbivore,
            params: &HERBIVORE,
            stats: Stats::new_default(),
        }
    }

    pub fn from(stats: Stats) -> Herbivore<'a> {
        Herbivore {
            species: Species::Herbivore,
            params: &HERBIVORE,
            stats,
        }
    }

    pub fn procreate(&mut self, count_in_cell: u32) -> Option<Self> {
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

        self.stats().weight -= amount_eaten * self.params().beta;
        self.update_fitness();
        amount_eaten
    }
}

#[derive(PartialEq, Debug)]
pub struct Carnivore<'a> {
    species: Species,
    params: &'a Parameters,
    stats: Stats,
}

impl AnimalTrait for Carnivore<'_> {
    fn stats(&mut self) -> &mut Stats {
        &mut self.stats
    }

    fn params(&self) -> &Parameters {
        &self.params
    }
}

impl<'a> Carnivore<'a> {
    pub fn new() -> Carnivore<'a> {
        Carnivore {
            species: Species::Carnivore,
            params: &CARNIVORE,
            stats: Stats::new_default(),
        }
    }

    pub fn from(stats: Stats) -> Carnivore<'a> {
        Carnivore {
            species: Species::Carnivore,
            params: &HERBIVORE,
            stats,
        }
    }

    pub fn procreate(&mut self, count_in_cell: u32) -> Option<Self> {
        if let Some(newborn_weight) = self.get_birthweight(count_in_cell) {
            let stats = Stats::from(0, newborn_weight);
            return Some(Carnivore::from(stats));
        }

        None
    }

    pub fn feeding(&mut self, herb_sorted_lowest_fitness: Vec<&mut Herbivore>) {
        let delta_phi_max = self.params().delta_phi_max;
        let mut amount_eaten: f32 = 0.0;

        for herbivore in herb_sorted_lowest_fitness {
            if amount_eaten >= self.params().f {
                break;
            }

            let diff_fitness = self.stats().fitness - herbivore.stats.fitness;

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

            let desired_food = self.params.f - amount_eaten;
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
mod test {
    use super::*;

    #[test]
    fn create_herb() {
        let expected = Herbivore::new();

        assert_eq!(
            expected,
            Herbivore {
                species: Species::Herbivore,
                params: &HERBIVORE,
                stats: Stats::new_default()
            }
        )
    }
    #[test]
    fn create_carn() {
        let expected = Carnivore::new();

        assert_eq!(
            expected,
            Carnivore {
                species: Species::Carnivore,
                params: &CARNIVORE,
                stats: Stats::new_default()
            }
        )
    }
}
