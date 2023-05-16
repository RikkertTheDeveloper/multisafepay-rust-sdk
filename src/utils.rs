pub struct RestfulUrlBuilder {
    url: Option<String>,
    parameter: Option<String>,
}

impl RestfulUrlBuilder {
    pub fn new() -> Self {
        Self {
            url: None,
            parameter: None,
        }
    }

    pub fn url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }

    pub fn parameter(mut self, parameter: String) -> Self {
        self.parameter = Some(parameter);
        self
    }

    pub fn format(&self) -> String {
        format!(
            "{}/{}",
            self.url.as_ref().unwrap(),
            self.parameter.as_ref().unwrap()
        )
    }
}
