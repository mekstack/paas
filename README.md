# SaaS

Service as a Service services

## Services

### Blau: **Bl**azingly fast **Au**th

Provides an HTTP endpoint `/login` that authorizes user via OAuth provider and
returns a signed JWT token that can be used to access other microservices.

#### Configuration

Configuration is performed by setting following environment variables:

- `SAAS_HTTP_PORT`: port that the actix web server will listen on (default: 80)
- `SAAS_CLIENT_ID`: OAuth client ID
- `SAAS_CLIENT_SECRET`: OAuth client secret
- `SAAS_ISSUER_URL`: OAuth issuer URL
- `SAAS_JWT_SECRET_KEY`: secret key for signing users JWT access tokens
- `SAAS_SECRET_KEY`: secret key for cookie encryption for session storage
- `SAAS_REDIRECT_URLS`: space-separated list of allowed redirect URLs after successful login
