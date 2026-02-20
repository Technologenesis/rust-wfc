pub struct WorldObjectLinguistics {
    pub name: String,
    pub definite_description: String,
    pub indefinite_description: String,
    pub pronoun: String,
}

impl WorldObjectLinguistics {
    pub fn new(name: String, definite_description: String, indefinite_description: String, pronoun: String) -> Self {
        Self { name, definite_description, indefinite_description, pronoun }
    }
}