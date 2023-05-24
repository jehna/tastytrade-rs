use reqwest::header;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use reqwest::ClientBuilder;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::api::base::Result;
use crate::api::base::TastyApiResponse;
use crate::api::base::TastyError;
use crate::api::login::LoginCredentials;
use crate::api::login::LoginResponse;

use reqwest_inspect_json::InspectJson;

pub const BASE_URL: &str = "https://api.cert.tastyworks.com";

#[derive(Debug, Clone)]
pub struct TastyTrade {
    client: reqwest::Client,
    pub(crate) session_token: String,
}

impl TastyTrade {
    pub async fn login(login: &str, password: &str, remember_me: bool) -> Result<Self> {
        let client = reqwest::Client::new();
        let resp = client
            .post(format!("{BASE_URL}/sessions"))
            .json(&LoginCredentials {
                login,
                password,
                remember_me,
            })
            .send()
            .await?;
        let json = resp
            .inspect_json::<TastyApiResponse<LoginResponse>, TastyError>(|text| println!("{text}"))
            .await?;
        let response = match json {
            TastyApiResponse::Success(s) => Ok(s),
            TastyApiResponse::Error { error } => Err(error),
        }?
        .data;

        let mut headers = HeaderMap::new();

        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(&response.session_token).unwrap(),
        );
        headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_str("application/json").unwrap(),
        );
        headers.insert(
            header::USER_AGENT,
            HeaderValue::from_str("tastytrade-rs").unwrap(),
        );
        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .expect("Could not create client");

        Ok(Self {
            client,
            session_token: response.session_token,
        })
    }

    pub async fn get<T: DeserializeOwned, U: AsRef<str>>(&self, url: U) -> Result<T> {
        let url = format!("{BASE_URL}{}", url.as_ref());

        let result = self
            .client
            .get(url)
            .send()
            .await?
            .inspect_json::<TastyApiResponse<T>, TastyError>(move |text| {
                println!("{text}");
            })
            //.json::<TastyApiResponse<T>>()
            .await?;

        match result {
            TastyApiResponse::Success(s) => Ok(s.data),
            TastyApiResponse::Error { error } => Err(error.into()),
        }
    }

    pub async fn post<R, P, U>(&self, url: U, payload: P) -> Result<R>
    where
        R: DeserializeOwned,
        P: Serialize,
        U: AsRef<str>,
    {
        let url = format!("{BASE_URL}{}", url.as_ref());
        let result = self
            .client
            .post(url)
            .body(serde_json::to_string(&payload).unwrap())
            .send()
            .await?
            .inspect_json::<TastyApiResponse<R>, TastyError>(move |text| {
                println!("{text}");
            })
            //.json::<TastyApiResponse<R>>()
            .await?;

        match result {
            TastyApiResponse::Success(s) => Ok(s.data),
            TastyApiResponse::Error { error } => Err(error.into()),
        }
    }
}
