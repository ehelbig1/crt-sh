use reqwest;
use url;
use async_trait::async_trait;

mod error;
mod model;

#[async_trait]
pub trait Datasource {
    async fn search_identity(self, identity: &str) -> Result<Vec<model::Certificate>, error::Error>;
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

#[async_trait]
impl Datasource for CrtShDatasource {
    async fn search_identity(self, identity: &str) -> Result<Vec<model::Certificate>, error::Error> {
        let url = format!("{}?q={}&output=json", &self.base_url, &identity);
        let response = reqwest::get(&url).await;

        let data = match response {
            Ok(response) => response.json::<Vec<model::Certificate>>().await,
            Err(_) => return Err(error::Error::RequestError),
        };

        match data {
            Ok(certificates) => Ok(certificates),
            Err(_) => return Err(error::Error::ParseError),
        }
    }
}
