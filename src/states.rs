#[derive(Clone, Copy, Default, Debug)]
pub enum MoveState {
    Moved,
    #[default]
    Not,
}

#[derive(Clone, Copy, Default, Debug)]
pub enum Entity {
    #[default]
    None,
    Predator(u32),
    Prey(u32),
}

pub struct SimArrays {
    pub entity_arr: [[Entity; 50]; 50],
    pub moved_arr: [[MoveState; 50]; 50],
    pub energy_arr: [[u16; 50]; 50],
    pub turn_arr: [[u16; 50]; 50],
}

impl Default for SimArrays {
    fn default() -> Self {
        Self {
            entity_arr: [[Default::default(); 50]; 50],
            moved_arr: [[Default::default(); 50]; 50],
            energy_arr: [[Default::default(); 50]; 50],
            turn_arr: [[Default::default(); 50]; 50],
        }
    }
}