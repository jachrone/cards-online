use std::fmt;

// A public struct with a public field of generic type `T`
#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub player_id: i32,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "name: {}\n", self.name)
    }
}
