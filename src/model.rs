use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct DataPoint {
    pub id: usize,
    pub name: String,
    pub description: String,
}

impl DataPoint {
    pub fn new(id: usize, name: String, description: String) -> Self {
        DataPoint {
            id,
            name,
            description
        }
    }
}

#[derive(Serialize,Deserialize)]
pub struct Application {
    pub id: usize,
    pub name: String,
    pub description: String,
}

impl Application {
    pub fn new(id: usize, name: String, description: String) -> Self {
        Application {
            id,
            name,
            description
        }
    }
}