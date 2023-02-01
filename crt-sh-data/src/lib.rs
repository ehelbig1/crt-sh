use reqwest;
use url;

mod error;
mod model;

pub trait Datasource {
    fn search_identity(self, identity: &str) -> Result<Vec<model::Certificate>, error::Error>;
}

pub struct CrtShDatasource {
    base_url: url::Url,
}

impl CrtShDatasource {
    pub fn new() -> Self {
        Self {
            base_url: url::Url::parse("https://crt.sh").unwrap(),
        }
    }
}

impl Datasource for CrtShDatasource {
    fn search_identity(self, identity: &str) -> Result<Vec<model::Certificate>, error::Error> {
        let url = format!("{}?q={}&output=json", &self.base_url, &identity);
        let response = reqwest::blocking::get(&url);

        let data = match response {
            Ok(response) => response.json::<Vec<model::Certificate>>(),
            Err(_) => return Err(error::Error::RequestError),
        };

        match data {
            Ok(certificates) => Ok(certificates),
            Err(_) => return Err(error::Error::ParseError),
        }
    }
}
