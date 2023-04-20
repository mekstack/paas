const JWT_TTL_SECONDS: u64 = 600;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub username: String,
    pub exp: u64,
}

#[derive(Clone)]
pub struct JwtEncoder {
    secret_key: String,
}

impl JwtEncoder {
    pub fn new(secret_key: String) -> Self {
        JwtEncoder { secret_key }
    }

    pub fn encode(&self, username: String) -> Result<String, jsonwebtoken::errors::Error> {
        let exp = jsonwebtoken::get_current_timestamp() + JWT_TTL_SECONDS;

        let claims = Claims { username, exp };
        jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(self.secret_key.as_ref()),
        )
    }
}
