pub struct Client {
    api_key: String,
}

impl Client {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    pub fn verify(&self) -> bool {
        !self.api_key.is_empty()
    }
}
