mod animals_params {
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
        pub F: f32,
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
        F: 10.0,
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
        F: 10.0,
        delta_phi_max: 10.0,
    };

    #[derive(PartialEq, Debug)]
    pub struct Stats {
        pub age: u32,
        pub weight: f32,
    }

    impl Stats {
        pub fn new_default() -> Stats {
            Stats {
                age: 5,
                weight: 20.0,
            }
        }
    }
}

use animals_params::{Parameters, Stats, CARNIVORE, HERBIVORE};

pub trait AnimalTrait {
    fn procreate(&self, count_in_cell: u32);

    fn calc_fitness(&self);

    fn update_fitness(&self);

    fn aging(&self);

    fn loss_of_weight(&self);

    fn death(&self);

    fn migrate(&self);
}

#[derive(PartialEq, Debug)]
pub struct Herbivore {
    params: Parameters,
    stats: Stats,
}

impl Herbivore {
    pub fn new() -> Herbivore {
        Herbivore {
            params: HERBIVORE,
            stats: Stats::new_default(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Carnivore {
    params: Parameters,
    stats: Stats,
}

impl Carnivore {
    pub fn new() -> Carnivore {
        Carnivore {
            params: CARNIVORE,
            stats: Stats::new_default(),
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
                params: HERBIVORE,
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
                params: CARNIVORE,
                stats: Stats::new_default()
            }
        )
    }
}
