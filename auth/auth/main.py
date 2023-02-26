import datetime

import jwt
from authlib.integrations.flask_client import OAuth
from flask import Flask, make_response, redirect, url_for, logging

from auth.util import register_oauth_providers_from_config, is_user_authorized
from auth.log import configure_default_logger

configure_default_logger()

app = Flask("auth")
app.logger = logging.create_logger(app)
app.config.from_object("config")

oauth = OAuth(app)
register_oauth_providers_from_config(oauth, app.config)


@app.route("/login/<provider>")
def login(provider):
    if provider not in app.config["OAUTH_PROVIDERS"]:
        return 501

    redirect_uri = url_for(f"auth", provider=provider, _external=True)
    return oauth.create_client(provider).authorize_redirect(redirect_uri)


@app.route("/auth/<provider>")
def auth(provider):
    token = oauth.create_client(provider).authorize_access_token()
    userinfo = token["userinfo"]

    if is_user_authorized(userinfo, provider, app.config):
        app.logger.info("Login success for user %s", userinfo["email"])
    else:
        app.logger.info("Login failure for user %s", userinfo["email"])
        return 403

    jwt_token = jwt.encode(
        {
            "email": userinfo["email"],
            "exp": datetime.datetime.utcnow() + datetime.timedelta(days=1),
        },
        app.config["JWT_SECRET_KEY"],
    )
    response = make_response(redirect("/"))
    response.set_cookie("accessToken", jwt_token)
    return response
