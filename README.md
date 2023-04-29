# SaaS

Service as a Service services

## Services

### Blau: **Bl**azingly fast **Au**th

Provides an HTTP endpoint `/login` that authorizes user via OAuth provider and
returns a signed JWT token that can be used to access other microservices.

#### Configuration

Configuration is performed by setting following environment variables:

- `VPNAAS_HTTP_PORT`: port that the actix web server will listen on (default: 80)
- `VPNAAS_CLIENT_ID`: OAuth client ID
- `VPNAAS_CLIENT_SECRET`: OAuth client secret
- `VPNAAS_ISSUER_URL`: OAuth issuer URL
- `VPNAAS_JWT_SECRET_KEY`: secret key for signing users JWT access tokens
- `VPNAAS_SECRET_KEY`: secret key for cookie encryption for session storage
- `VPNAAS_REDIRECT_URLS`: space-separated list of allowed redirect URLs after successful login
