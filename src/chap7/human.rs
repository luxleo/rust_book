pub enum Race {
    Asian,
    African
}
pub struct Human {
    pub race: Race,
    pub name: String,
    age: u8,
}
impl Human {
    pub fn new(race: Race, name: String, age: u8) -> Self {
        Human { race, name, age }
    }
    fn age(&self) -> u8{
        self.age
    }
}