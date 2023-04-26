mod config;
mod jwt;
mod routes;
mod tests;
mod util;

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
    env_logger::init();

    let config = Config::from_env();
    let util = util::Util::new(config.redirect_urls);
    let jwt = jwt::JwtEncoder::new(config.jwt_secret_key);

    let provider_metadata =
        CoreProviderMetadata::discover_async(config.issuer_url, async_http_client)
            .await
            .expect("Provider metadata discovery failed");

    log::info!("Provider metadata successfully discovered");

    let client = CoreClient::from_provider_metadata(
        provider_metadata,
        config.client_id,
        config.client_secret,
    );

    let server_url = format!("0.0.0.0:{}", config.server_port);

    log::info!("Starting actix server on {}", server_url);
    HttpServer::new(move || {
        App::new()
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                config.secret_key.clone(),
            ))
            .app_data(web::Data::new(util.clone()))
            .app_data(web::Data::new(client.clone()))
            .app_data(web::Data::new(jwt.clone()))
            .service(login)
            .service(redirect_uri)
    })
    .bind(server_url)?
    .run()
    .await
}
