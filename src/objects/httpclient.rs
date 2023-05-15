use minreq::{self, Response, Error};

pub struct HttpClient {
    url: String,
    method: String,
}

impl HttpClient {
    pub fn new(url: String, method: String) -> Self {
        Self { url, method }
    }

    pub fn post(&self, payload: String) -> Result<String, Error> {
        let response = minreq::post(self.url.to_owned())
        .with_body(payload)
        .send()?;

        let parsed = response.as_str()?;
        Ok(parsed.to_owned())
    }

    pub fn get(&self) -> Result<String, Error> {
        let response: Response = minreq::get(self.url.to_owned()).send()?;
        let parsed = response.as_str()?;

        Ok(parsed.to_owned())
    }
}
