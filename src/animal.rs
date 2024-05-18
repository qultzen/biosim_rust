mod animals_params {
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
        pub DeltaPhiMax: f32,
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
        DeltaPhiMax: 0.0,
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
        DeltaPhiMax: 10.0,
    };
}

use animals_params::{CARNIVORE, HERBIVORE};
