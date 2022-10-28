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