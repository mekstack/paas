use actix_web::cookie;
use openidconnect::{ClientId, ClientSecret, IssuerUrl};
use std::env;

pub struct Config {
    pub jwt_secret_key: String,
    pub client_id: ClientId,
    pub client_secret: Option<ClientSecret>,
    pub issuer_url: IssuerUrl,
    pub secret_key: cookie::Key,
    pub server_port: String,
}

impl Config {
    pub fn from_env() -> Config {
        Config {
            server_port: get_env_var_or_default("PORT", "80"),
            client_id: ClientId::new(get_env_var("CLIENT_ID")),
            client_secret: Some(ClientSecret::new(get_env_var("CLIENT_SECRET"))),
            issuer_url: IssuerUrl::new(get_env_var("ISSUER_URL")).unwrap(),
            jwt_secret_key: get_env_var("JWT_SECRET_KEY"),
            secret_key: cookie::Key::from(get_env_var("SECRET_KEY").as_bytes()),
        }
    }
}

fn get_env_var(var_name: &str) -> String {
    env::var(var_name).unwrap_or_else(|_| panic!("Environment variable {} is unset", var_name))
}

fn get_env_var_or_default(var_name: &str, default_value: &str) -> String {
    env::var(var_name).unwrap_or_else(|_| default_value.to_string())
}
