# pylint: disable=too-few-public-methods
import jwt
import pytest
from auth.app import app, oauth
from werkzeug.http import parse_cookie


class MockOAuthClient:
    def __init__(self, name):
        self.userinfo = {
            "iss": "https://accounts.google.com",
            "azp": "foo",
            "aud": "foo",
            "sub": "111",
            "email": "test@gmail.com",
            "email_verified": True,
            "at_hash": "foo",
            "nonce": "foo",
            "name": "John Doe",
            "picture": "https://google.com",
            "given_name": "Jonh",
            "family_name": "Doe",
            "locale": "en-US",
            "iat": 0,
            "exp": 0,
        }
        if name == "google":
            self.userinfo["hd"] = "test.hd"

    def authorize_access_token(self):
        token = {
            "access_token": "foo",
            "expires_in": 0,
            "token_type": "Bearer",
            "id_token": jwt.encode(self.userinfo, "secret"),
            "expires_at": 0,
            "userinfo": self.userinfo,
        }

        return token


@pytest.fixture(autouse=True)
def mock_oauth_client(monkeypatch):
    def mock_create_client(name):
        return MockOAuthClient(name)

    def empty_init(_):
        pass

    monkeypatch.setattr(oauth, "__init__", empty_init)
    monkeypatch.setattr(oauth, "create_client", mock_create_client)


@pytest.fixture(autouse=True)
def mock_flask_config():
    app.config["SECRET_KEY"] = "secret"
    app.config["JWT_SECRET_KEY"] = "secret"


def test_login_basic():
    response = app.test_client().get("/auth/basic")

    assert response.status_code == 302


def test_login_logging(caplog):
    app.test_client().get("/auth/basic")

    assert "Login success for user test@gmail.com" in caplog.text


def test_login_google(caplog):
    app.config["GOOGLE_AUTHORIZE_PARAMS"] = {"hd": "test.hd"}
    response = app.test_client().get("/auth/google")

    assert response.status_code == 302
    assert "Login success for user test@gmail.com" in caplog.text


def test_login_google_fail(caplog):
    app.config["GOOGLE_AUTHORIZE_PARAMS"] = {"hd": "not.test.hd"}
    response = app.test_client().get("/auth/google")

    assert response.status_code == 403
    assert "Login failure for user test@gmail.com" in caplog.text


def test_login_jwt():
    response = app.test_client().get("/auth/basic")
    cookies = response.headers.getlist("Set-Cookie")
    jwt_cookie = next((cookie for cookie in cookies if "accessToken" in cookie), None)

    assert jwt_cookie is not None

    jwt_cookie_attrs = parse_cookie(jwt_cookie)

    assert "Secure" in jwt_cookie_attrs
    assert "HttpOnly" in jwt_cookie_attrs
    assert jwt_cookie_attrs["SameSite"] == "Lax"
    assert jwt_cookie_attrs["Max-Age"] == str(app.config["JWT_MAX_AGE_SECONDS"])
