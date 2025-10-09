use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub exp: usize,
}

#[derive(Clone)]
pub struct Auth0Config {
    pub domain: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

impl Auth0Config {
    pub fn from_env() -> Self {
        Self {
            domain: std::env::var("AUTH0_DOMAIN").expect("AUTH0_DOMAIN must be set"),
            client_id: std::env::var("AUTH0_CLIENT_ID").expect("AUTH0_CLIENT_ID must be set"),
            client_secret: std::env::var("AUTH0_CLIENT_SECRET").expect("AUTH0_CLIENT_SECRET must be set"),
            redirect_uri: std::env::var("AUTH0_REDIRECT_URI").unwrap_or_else(|_| "http://localhost:8080/callback".to_string()),
        }
    }
    
    pub fn auth_url(&self) -> String {
        format!(
            "https://{}/authorize?response_type=code&client_id={}&redirect_uri={}&scope=openid%20profile%20email&connection=google-oauth2",
            self.domain, self.client_id, self.redirect_uri
        )
    }
}
