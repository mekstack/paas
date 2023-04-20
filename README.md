# SaaS

Service as a Service services

## Services

### Blau

> **Bl**azingly fast **Au**th

Provides an HTTP endpoint `/login/<provider>` that authorizes user via OAuth
provider and returns a signed JWT token.

#### Configuration

Configuration is performed by setting following variables.

-   **FLASK_OAUTH_PROVIDERS**: list of provider names.

    Example: `FLASK_OAUTH_PROVIDERS = ["google", "keycloak"]`

    Each OAuth provider is configured with env variables formatted as
    **`FLASK_{provider_name}_{option}`**.
    **`provider_name` must be uppercase**.
    Required `option`s are `CLIENT_ID`, `CLIENT_SECRET` and `METADATA_URL`.
    Example: ` FLASK_GOOGLE_CLIENT_ID = client_id`

-   **FLASK_SECRET_KEY**: string that sets app.secret_key

-   **FLASK_JWT_SECRET_KEY**: string for JWT signing

#### Run tests

    cd auth
    cargo test
