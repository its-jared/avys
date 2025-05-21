pub struct Block {
    pub id: usize, 
    pub texture_location: String,
}

impl Block {
    pub fn new(id: usize, texture_location: String) -> Self {
        Self {
            id,
            texture_location,
        }
    }
}