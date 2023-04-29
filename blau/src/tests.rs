#[cfg(test)]

mod tests {
    use crate::config::Config;
    use crate::jwt;
    use jsonwebtoken::{decode, DecodingKey, Validation};

    use openidconnect::{ClientId, ClientSecret, IssuerUrl};
    use std::env;

    #[test]
    fn test_config_loading() {
        env::set_var("BLAU_PORT", "8080");
        env::set_var("CLIENT_ID", "test_client_id");
        env::set_var("CLIENT_SECRET", "test_client_secret");
        env::set_var("ISSUER_URL", "https://example.com");
        env::set_var("JWT_SECRET_KEY", "test_jwt_secret_key");
        env::set_var("SECRET_KEY", String::from_utf8([65; 64].to_vec()).unwrap());

        let config = Config::from_env();

        assert_eq!(config.http_port, "8080");
        assert_eq!(
            config.client_id,
            ClientId::new("test_client_id".to_string())
        );
        assert_eq!(
            config.openid_client_secret.unwrap().secret(),
            ClientSecret::new("test_client_secret".to_string()).secret()
        );
        assert_eq!(
            config.openid_issuer_url,
            IssuerUrl::new("https://example.com".to_string()).unwrap()
        );
        assert_eq!(config.jwt_secret_key, "test_jwt_secret_key");
        assert_eq!(
            config.actix_secret_key.master(),
            actix_web::cookie::Key::from(&[65; 64]).master()
        );
    }

    #[test]
    fn test_jwt_encoding_decoding() {
        let jwt_secret_key = "secret_key";
        let jwt_encoder = jwt::JwtEncoder::new(jwt_secret_key.to_string());
        let username = "test_user";

        let token = jwt_encoder.encode(username.to_string()).unwrap();

        let decoded_claims = decode::<jwt::Claims>(
            &token,
            &DecodingKey::from_secret(jwt_secret_key.as_ref()),
            &Validation::default(),
        )
        .unwrap();

        assert_eq!(decoded_claims.claims.username, username);
    }
}
