use actix_web::cookie;
use openidconnect::{ClientId, ClientSecret, IssuerUrl};
use std::env;

pub struct Config {
    pub http_port: String,
    pub openid_client_id: ClientId,
    pub openid_client_secret: Option<ClientSecret>,
    pub openid_issuer_url: IssuerUrl,
    pub jwt_secret_key: String,
    pub actix_secret_key: cookie::Key,
    pub redirect_urls: Vec<String>,
}

impl Config {
    pub fn from_env() -> Config {
        Config {
            http_port: get_env_var_or_default("SAAS_HTTP_PORT", "80"),
            openid_client_id: ClientId::new(get_env_var("SAAS_CLIENT_ID")),
            openid_client_secret: Some(ClientSecret::new(get_env_var("SAAS_CLIENT_SECRET"))),
            openid_issuer_url: IssuerUrl::new(get_env_var("SAAS_ISSUER_URL")).unwrap(),
            jwt_secret_key: get_env_var("SAAS_JWT_SECRET_KEY"),
            actix_secret_key: cookie::Key::from(get_env_var("SAAS_SECRET_KEY").as_bytes()),
            redirect_urls: get_env_var("SAAS_REDIRECT_URLS")
                .split_whitespace()
                .filter(|url| !url.is_empty())
                .map(|url| url.to_string())
                .collect::<Vec<String>>(),
        }
    }
}

fn get_env_var(var_name: &str) -> String {
    env::var(var_name).unwrap_or_else(|_| panic!("Environment variable {} is unset", var_name))
}

fn get_env_var_or_default(var_name: &str, default_value: &str) -> String {
    env::var(var_name).unwrap_or_else(|_| default_value.to_string())
}
