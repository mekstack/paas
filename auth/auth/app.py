import logging

from authlib.integrations.flask_client import OAuth
from flask import Flask, make_response, redirect, url_for

from auth import util
from auth.log import configure_logging

app = Flask("auth")
configure_logging()
log = logging.getLogger("auth")

app.config.from_prefixed_env()
app.config.from_object("config")

oauth = OAuth(app)
util.register_oauth_providers_from_config(oauth, app.config)


@app.route("/login/<provider>")
def login(provider):
    client = oauth.create_client(provider)
    if client is None:
        return "501 Not implemented", 501

    url = url_for("redirect_uri", provider=provider, _external=True)
    return client.authorize_redirect(url)


@app.route("/redirect_uri/<provider>")
def redirect_uri(provider):
    client = oauth.create_client(provider)
    if client is None:
        return "501 Not implemented", 501

    token = client.authorize_access_token()
    userinfo = token["userinfo"]

    if util.is_user_authorized(userinfo, provider, app.config):
        log.info("Login success for user %s", userinfo["email"])
    else:
        log.info("Login failure for user %s", userinfo["email"])
        return "403 Forbidden", 403

    user_jwt = util.generate_user_jwt(userinfo, app.config)
    response = make_response(redirect("/"))
    response.set_cookie(
        "accessToken",
        user_jwt,
        httponly=True,
        secure=True,
        samesite="Lax",
        max_age=app.config["JWT_MAX_AGE_SECONDS"],
    )
    return response
