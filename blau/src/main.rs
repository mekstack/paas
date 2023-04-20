mod config;
mod jwt;
mod routes;
mod tests;

use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{web, App, HttpServer};
use openidconnect::{
    core::{CoreClient, CoreProviderMetadata},
    reqwest::async_http_client,
};

use config::Config;
use routes::{login, redirect_uri};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env();
    let jwt = jwt::JwtEncoder::new(config.jwt_secret_key);

    let provider_metadata =
        CoreProviderMetadata::discover_async(config.issuer_url, async_http_client)
            .await
            .expect("Provider metadata discovery failed");

    let client = CoreClient::from_provider_metadata(
        provider_metadata,
        config.client_id,
        config.client_secret,
    );

    HttpServer::new(move || {
        App::new()
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                config.secret_key.clone(),
            ))
            .app_data(web::Data::new(client.clone()))
            .app_data(web::Data::new(jwt.clone()))
            .service(login)
            .service(redirect_uri)
    })
    .bind(format!("0.0.0.0:{}", config.server_port))?
    .run()
    .await
}
