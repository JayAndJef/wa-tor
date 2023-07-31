use std::default;
use crate::states::{SimArrays, Entity};
use rand::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct EntityOverflowError;

pub struct Sim {
    arrays: SimArrays
}

impl Default for Sim {
    fn default() -> Self {
        Self::new()
    }
}

impl Sim {
    pub fn new() -> Self {
        Self { arrays: SimArrays::default() }
    }

    pub fn fill_states(&mut self, pred_count: u32, prey_count: u32) -> Result<&mut Self, EntityOverflowError> {

        let mut rng = thread_rng();
        
        if (pred_count + prey_count) > (50*50) {
            return Err(EntityOverflowError);
        }

        let (mut x, mut y) = (0, 0);

        for _ in 0..prey_count {
            loop {
                (x, y) = (rng.gen_range(0..50), rng.gen_range(0..50));
                if let Entity::None = self.arrays.entity_arr[x][y] {
                    self.arrays.entity_arr[x][y] = Entity::Prey;
                    break;
                } else {
                    continue;
                }
            }   
        }

        for _ in 0..pred_count {
            loop {
                (x, y) = (rng.gen_range(0..50), rng.gen_range(0..50));
                if let Entity::None = self.arrays.entity_arr[x][y] {
                    self.arrays.entity_arr[x][y] = Entity::Predator;
                    break;
                } else {
                    continue;
                }
            }   
        }

        Ok(self)
    }
}


#[cfg(test)]
mod tests {
    use crate::sim::EntityOverflowError;

    use super::Sim;

    #[test]
    fn always_passes() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic]
    fn test_fill_states_err() {
        let _ = Sim::default().fill_states(2000, 2000).unwrap();
    }
}
