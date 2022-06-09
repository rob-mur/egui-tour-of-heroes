use std::borrow::Cow;

pub struct Hero {
    pub id: i32,
    pub name: Cow<'static, str>,
}

impl Default for Hero {
    fn default() -> Self {
        Self {
            id: 1,
            name: Cow::Borrowed("Windstorm"),
        }
    }
}
