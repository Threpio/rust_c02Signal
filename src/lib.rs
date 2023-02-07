use std::borrow::Borrow;
use std::error::Error;
use url::Url;
use serde::Serialize;
use serde::Deserialize;


// Create the client for the c02 signal api
// Rate limiting of 30ph  and 1 ps
// Take an API key from the environment

pub struct Client {
    client: reqwest::Client,
    api_key: String,
    endpoint: String,
}

impl Client {
    pub fn new(api_key: String) -> Self {
        if api_key.is_empty() {
            panic!("API key is empty");
        }
        Client {
            client: reqwest::Client::new(),
            api_key,
            endpoint: "https://api.co2signal.com/v1/latest".to_string(),
        }
    }
    pub async fn latest_cc(&self, country_code: String) -> Result<CcResponse, Box<dyn Error>> {
        let url = Url::parse(self.endpoint.borrow())?;

        let req = self.client
            .get(url)
            .query(&[("countryCode", country_code)])
            .header(
                "auth-token",
                format!("{}", self.api_key)
            )
            .send()
            .await?;

        match req.status() {
            reqwest::StatusCode::OK => {
                let resp = req.text().await?;
                let resp: CcResponse = serde_json::from_str(&resp)?;
                Ok(resp)
            }
            _ => {
                let resp = req.text().await?;
                Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, resp)))
            }
        }
    }
    pub async fn latest_gc(&self, lat: f32, lon: f32) -> Result<CcResponse, Box<dyn Error>> {
        let url = Url::parse(self.endpoint.borrow())?;

        let req = self.client
            .get(url)
            .query(&[("lat", lat), ("lon", lon)])
            .header(
                "auth-token",
                format!("{}", self.api_key)
            )
            .send()
            .await?;

        match req.status() {
            reqwest::StatusCode::OK => {
                let resp = req.text().await?;
                let resp: CcResponse = serde_json::from_str(&resp)?;
                Ok(resp)
            }
            _ => {
                let resp = req.text().await?;
                Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, resp)))
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Units {
    #[serde(rename = "carbonIntensity")]
    pub carbon_intensity: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub datetime: String,
    #[serde(rename = "carbonIntensity")]
    pub carbon_intensity: i64,
    #[serde(rename = "fossilFuelPercentage")]
    pub fossil_fuel_percentage: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CcResponse {
    pub _disclaimer: String,
    pub status: String,
    #[serde(rename = "countryCode")]
    pub country_code: String,
    pub data: Data,
    pub units: Units,
}
