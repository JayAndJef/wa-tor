use std::default;
use crate::states::{SimArrays, Entity, MoveState};
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

        for id in 0..prey_count {
            loop {
                (x, y) = (rng.gen_range(0..50), rng.gen_range(0..50));
                if let Entity::None = self.arrays.entity_arr[x][y] {
                    self.arrays.entity_arr[x][y] = Entity::Prey(id);
                    break;
                } else {
                    continue;
                }
            }   
        }

        for id in 0..pred_count {
            loop {
                (x, y) = (rng.gen_range(0..50), rng.gen_range(0..50));
                if let Entity::None = self.arrays.entity_arr[x][y] {
                    self.arrays.entity_arr[x][y] = Entity::Predator(id);
                    break;
                } else {
                    continue;
                }
            }   
        }

        Ok(self)
    }

    fn move_entity(&mut self, row: usize, col: usize) {

        let mut rng = thread_rng();

        match self.arrays.entity_arr[row][col] {
            Entity::None => panic!("tried to move an empty square"),
            Entity::Predator(id) => { //TODO add energy
                if let Entity::Prey(_) = self.arrays.entity_arr[row+1][col] {
                    self.arrays.entity_arr[row+1][col] = Entity::Predator(id);
                    self.arrays.entity_arr[row][col] = Entity::None;
                    self.arrays.moved_arr[row+1][col] = MoveState::Moved;
                    return;
                }
                if let Entity::Prey(_) = self.arrays.entity_arr[row-1][col] {
                    self.arrays.entity_arr[row-1][col] = Entity::Predator(id);
                    self.arrays.entity_arr[row][col] = Entity::None;
                    self.arrays.moved_arr[row-1][col] = MoveState::Moved;
                    return;
                }
                if let Entity::Prey(_) = self.arrays.entity_arr[row][col+1] {
                    self.arrays.entity_arr[row][col+1] = Entity::Predator(id);
                    self.arrays.entity_arr[row][col] = Entity::None;
                    self.arrays.moved_arr[row][col+1] = MoveState::Moved;
                    return;
                }
                if let Entity::Prey(_) = self.arrays.entity_arr[row][col-1] {
                    self.arrays.entity_arr[row][col-1] = Entity::Predator(id);
                    self.arrays.entity_arr[row][col] = Entity::None;
                    self.arrays.moved_arr[row][col-1] = MoveState::Moved;
                    return;
                }

                loop {
                    let x_move: i16 = rng.gen_range(-1..=1);
                    let y_move: i16 = rng.gen_range(-1..=1);
                    if let Entity::None = self.arrays.entity_arr[(col as i16 + x_move) as usize][(row as i16 + x_move) as usize] {
                        self.arrays.entity_arr[(col as i16 + x_move) as usize][(row as i16 + x_move) as usize] = Entity::Predator(id);
                        self.arrays.entity_arr[row][col] = Entity::None;
                        self.arrays.moved_arr[(col as i16 + x_move) as usize][(row as i16 + x_move) as usize] = MoveState::Moved;
                        break;
                    }
                }
            },
            Entity::Prey(_) => todo!()
        } 
        
    }

    fn update(&mut self) {
        for row_num in 0..50 {
            for col_num in 0..50 {
                if let Entity::Prey(id) = self.arrays.entity_arr[row_num][col_num] {
                    todo!();
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
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
