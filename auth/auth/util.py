import logging

log = logging.getLogger("auth.util")


def register_oauth_providers_from_config(oauth, config):
    for provider in config["OAUTH_PROVIDERS"]:
        oauth.register(
            name=provider,
            server_metadata_url=config[f"{provider}_METADATA_URL".upper()],
            client_kwargs={"scope": "openid email profile"},
        )
        log.info("Registered OAuth provider %s", provider)


def is_user_authorized(userinfo, provider, config):
    if provider == "google":
        if userinfo.get("hd") != config["GOOGLE_AUTHORIZE_PARAMS"]["hd"]:
            return False
    return True
