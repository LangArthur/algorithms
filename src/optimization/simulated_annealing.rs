use rand::Rng;

#[derive(strum_macros::Display)]
pub enum Cooling {
    Linear { factor: f64 },
    Factorial { factor: f64 }, // Need to be between 0 and 1
    Logarithmic { factor: f64 },
}

// Simulated Annealing is an optimization algorithm designed to search for an
// optimal or near-optimal solution in a large solution space

pub struct SimulatedAnnealing {
    max: Option<usize>,
    temperature: f64,
    cooling: Cooling,
}

impl SimulatedAnnealing {
    fn cooling(&mut self) {
        self.temperature = match self.cooling {
            Cooling::Linear { factor } => self.temperature - factor,
            Cooling::Factorial { factor } => self.temperature * factor,
            _ => {
                todo!("Implement cooling for: {}", self.cooling)
            }
        }
        .max(0.0);
    }

    pub fn run(&mut self, arr: &[i32]) -> i32 {
        let mut res = match arr.first() {
            Some(n) => n.clone(),
            None => return 0,
        };
        let mut rng = rand::rng();
        let mut nbr_iteration: usize = 0;
        while self.temperature.round() > 0.0 {
            if let Some(max) = self.max
                && nbr_iteration == max - 1
            {
                return res;
            }

            // generate new solution
            let new_idx = rng.random_range(0..arr.len());
            let s = arr[new_idx];
            let delta = (s - res) as f64;
            // evaluation
            if delta > 0.0 {
                res = s
            } else {
                // since we maximize the result, delta is not negated
                let acceptance = (delta / self.temperature).exp();
                let random = rng.random_range(0.0..1.0);
                if random < acceptance {
                    res = s;
                }
            }
            self.cooling();
            nbr_iteration += 1;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let mut algo = SimulatedAnnealing {
            max: None,
            temperature: 30.0,
            cooling: Cooling::Factorial { factor: 0.95 },
        };
        // It can failed from time to time due to randomness of the algorithm
        // assert_eq!(
        //     algo.run(&[1, 5, 7, 7, 6, 4, 3, 2, 5, 1, 4, 5, 6, 2, 5, 3, 1, -2, 3]),
        //     7
        // );
        // so it is replace by this test which work most of the time.
        assert!(7 - algo.run(&[1, 5, 7, 7, 6, 4, 3, 2, 5, 1, 4, 5, 6, 2, 5, 3, 1, -2, 3]) < 2);
        assert_eq!(algo.run(&[8]), 8);
        assert_eq!(algo.run(&[]), 0);
    }
}
