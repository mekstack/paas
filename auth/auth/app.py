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
    if provider not in app.config["OAUTH_REGISTERED_PROVIDERS"]:
        return "501 Not implemented", 501

    redirect_uri = url_for("auth", provider=provider, _external=True)
    return oauth.create_client(provider).authorize_redirect(redirect_uri)


@app.route("/auth/<provider>")
def auth(provider):
    token = oauth.create_client(provider).authorize_access_token()
    userinfo = token["userinfo"]

    if util.is_user_authorized(userinfo, provider, app.config):
        log.info("Login success for user %s", userinfo["email"])
    else:
        log.info("Login failure for user %s", userinfo["email"])
        return "403 Forbidden", 403

    user_jwt = util.generate_user_jwt(userinfo, app.config)
    response = make_response(redirect("/"))
    response.set_cookie("accessToken", user_jwt)
    return response
