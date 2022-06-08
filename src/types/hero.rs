pub struct Hero {
    pub id: i32,
    pub name: String,
}

impl Default for Hero {
    fn default() -> Self {
        Self {
            id: 1,
            name: "Windstorm".to_string(),
        }
    }
}
