use rand::Rng;

#[derive(strum_macros::Display)]
pub enum Cooling {
    Linear,
    Exponential,
    Logarithmic,
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
            Cooling::Linear => self.temperature - 1.0,
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
                // FIXME: acceptance is nearly never under 1.
                // The reason might be because we select a random index.
                let acceptance = (-(delta / self.temperature)).exp();
                let random = rng.random_range(0.0..1.0);
                println!(
                    "{}, acceptance: {acceptance}, random: {random}",
                    (-delta / self.temperature)
                );
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
            temperature: 100.0,
            cooling: Cooling::Linear,
        };
        assert_eq!(algo.run(&[1, 5, 7, 9, 6, 4, 3, 2, 5, 1]), 9);
        assert_eq!(algo.run(&[7, 9, 10, 8, -1, 8]), -1);
        assert_eq!(algo.run(&[8]), 8);
        assert_eq!(algo.run(&[]), 0);
    }
}
