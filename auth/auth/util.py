import datetime
import logging

import jwt

log = logging.getLogger("auth.util")


def register_oauth_providers_from_config(oauth, config):
    if config.get("OAUTH_PROVIDERS") is None:
        log.error("OAUTH_PROVIDERS is not defined")
        return

    for provider in config.get("OAUTH_PROVIDERS"):
        try:
            oauth.register(
                name=provider,
                client_id=config[f"{provider.upper()}_CLIENT_ID"],
                client_secret=config[f"{provider.upper()}_CLIENT_SECRET"],
                server_metadata_url=config[f"{provider.upper()}_METADATA_URL"],
                client_kwargs={"scope": "openid email profile"},
            )
            log.info("Registered OAuth provider %s", provider)
        except KeyError as err:
            log.error("Failed to register OAuth provider %s", provider, exc_info=err)


def generate_user_jwt(userinfo, config):
    return jwt.encode(
        {
            "email": userinfo["email"],
            "exp": datetime.datetime.utcnow() + datetime.timedelta(minutes=10),
        },
        config["JWT_SECRET_KEY"],
    )


def is_user_authorized(userinfo, provider, config):
    if provider == "google":
        if userinfo.get("hd") != config["GOOGLE_AUTHORIZE_PARAMS"]["hd"]:
            return False
    return True
