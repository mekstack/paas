# SaaS

Service as a Service services

## Services

### Blau: **Bl**azingly fast **Au**th

Provides an HTTP endpoint `/login` that authorizes user via OAuth provider and
sets a cookie with a signed JWT token that can be used to access microservices.

#### Configuration

Configuration is performed by setting following environment variables:

-   `BLAU_PORT`: port for web server to listen on
-   `CLIENT_ID`: OpenID client id
-   `CLIENT_SECRET`: OpenID client secret
-   `ISSUER_URL`: OpenID issuer url
-   `JWT_SECRET_KEY`: key for signing JWTs
-   `SECRET_KEY`: key for session persistence Cookie encryption

#### Run tests

    cd blau
    cargo test
