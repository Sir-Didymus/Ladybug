/// The main character in this project!
/// The Ladybug struct acts as the UCI client and can receive and handle UCI commands.
pub struct Ladybug {
    name: String,
    author: String,
}

impl Default for Ladybug {
    /// Constructs Ladybug.
    fn default() -> Self {
        Self {
            name: String::from("Ladybug 0.1.0"),
            author: String::from("\"cs-patzer\" - Felix O."),
        }
    }
}

