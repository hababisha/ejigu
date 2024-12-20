use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Course {
    pub name: String,
    pub absences: u32,
}

impl Course {
    pub fn new(name: String) -> Self {
        Self {
            name,
            absences: 0,
        }
    }
}
