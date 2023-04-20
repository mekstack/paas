use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use jsonwebtoken::get_current_timestamp;
// use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug)]
struct Claims {
    username: String,
    exp: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct AccessToken {
    email: String,
}

#[derive(Deserialize, Debug)]
pub struct AuthRequest {
    code: String,
    state: String,
}

use openidconnect::core::{
    CoreAuthenticationFlow, CoreClient, CoreIdTokenClaims, CoreProviderMetadata, CoreResponseType,
    CoreUserInfoClaims,
};
use openidconnect::{
    AccessTokenHash, AuthenticationFlow, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    IssuerUrl, Nonce, PkceCodeChallenge, RedirectUrl, Scope, TokenResponse,
};

use openidconnect::reqwest::async_http_client;

use actix_session::Session;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let provider_metadata = CoreProviderMetadata::discover_async(
        IssuerUrl::new("https://profile.miem.hse.ru/auth/realms/MIEM".to_string()).unwrap(),
        async_http_client,
    )
    .await
    .unwrap();

    let client = CoreClient::from_provider_metadata(
        provider_metadata,
        ClientId::new(env::var("KEYCLOAK_CLIENT_ID").unwrap().to_string()),
        Some(ClientSecret::new(
            env::var("KEYCLOAK_CLIENT_SECRET").unwrap().to_string(),
        )),
    )
    .set_redirect_uri(RedirectUrl::new("http://127.0.0.1:5000/redirect_uri".to_string()).unwrap());

    HttpServer::new(move || {
        App::new()
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                actix_web::cookie::Key::from(
                    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                        .as_bytes(),
                ),
            ))
            .data(client.clone())
            .service(login)
            .service(redirect_uri)
    })
    .bind("127.0.0.1:5000")
    .unwrap()
    .run()
    .await
}

#[get("/login")]
async fn login(client: web::Data<CoreClient>, session: Session) -> impl Responder {
    let (authorize_url, csrf_state, nonce) = client
        .authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .url();

    session.insert("csrf_state", csrf_state.clone()).unwrap();
    session.insert("nonce", nonce.clone()).unwrap();

    HttpResponse::TemporaryRedirect()
        .header("Location", authorize_url.to_string())
        .finish()
}

#[get("/redirect_uri")]
async fn redirect_uri(
    params: web::Query<AuthRequest>,
    client: web::Data<CoreClient>,
    session: Session,
) -> Result<HttpResponse, actix_web::Error> {
    let stored_state: String = session
        .get("csrf_state")
        .unwrap_or(None)
        .unwrap_or_default();
    if stored_state != params.state {
        return Err(actix_web::error::ErrorBadRequest("Invalid CSRF state"));
    }

    println!("{:#?}", params);
    let code = AuthorizationCode::new(params.code.clone());

    let token_response = client
        .exchange_code(code)
        .request_async(async_http_client)
        .await
        .unwrap();

    let id_token = token_response.id_token().unwrap();
    let nonce: Nonce = session.get("nonce").unwrap().unwrap();
    let claims: &CoreIdTokenClaims = id_token
        .claims(&client.id_token_verifier(), &nonce)
        .unwrap();

    println!("{:#?}", claims.email());

    let email = match claims.email() {
        Some(email) => email,
        None => return Err(actix_web::error::ErrorBadRequest("Error getting token")),
    };

    let username = email.split('@').next().unwrap().to_string();

    let exp = get_current_timestamp() + 10 * 60;

    let claims = Claims { username, exp };

    let jwt = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(env::var("JWT_SECRET_KEY").unwrap().as_ref()), // get in main
    )
    .unwrap();

    Ok(HttpResponse::TemporaryRedirect()
        .header("Location", "/")
        .cookie(
            actix_web::cookie::CookieBuilder::new("accessToken", jwt)
                .http_only(true)
                .secure(true)
                .same_site(actix_web::cookie::SameSite::Lax)
                .max_age(actix_web::cookie::time::Duration::minutes(10))
                .finish(),
        )
        .finish())
}
