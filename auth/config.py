import os

SECRET_KEY = os.getenv("FLASK_SECRET_KEY")

SESSION_COOKIE_SECURE = True
SESSION_COOKIE_HTTPONLY = True
SESSION_COOKIE_SAMESITE = "Lax"

OAUTH_PROVIDERS = ["google", "keycloak"]

GOOGLE_CLIENT_ID = os.getenv("GOOGLE_CLIENT_ID")
GOOGLE_CLIENT_SECRET = os.getenv("GOOGLE_CLIENT_SECRET")
GOOGLE_AUTHORIZE_PARAMS = {"hd": "miem.hse.ru"}  # Should not be used without validation of the returned ID token
GOOGLE_METADATA_URL = "https://accounts.google.com/.well-known/openid-configuration"

KEYCLOAK_CLIENT_ID = os.getenv("KEYCLOAK_CLIENT_ID")
KEYCLOAK_CLIENT_SECRET = os.getenv("KEYCLOAK_CLIENT_SECRET")

JWT_SECRET_KEY = os.getenv("JWT_SECRET_KEY")
