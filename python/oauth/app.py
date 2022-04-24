#!/usr/bin/env python
from urllib.parse import quote_plus

import requests
from flask import Flask, redirect, render_template, request, session
from requests.models import PreparedRequest

from config import client_id, client_secret, secret_key

app = Flask(__name__)
app.secret_key = secret_key

PORT = 5000
redirect_uri = quote_plus(f"http://localhost:{PORT}/github/auth/redirect")


def log_in_with_github_url() -> str:
    r = PreparedRequest()
    r.prepare(
        url="https://github.com/login/oauth/authorize",
        params={
            "client_id": client_id,
            "redirect_url": redirect_uri,
            "scope": "user:email",
        },
    )
    url = r.url
    assert isinstance(url, str)
    return url


def github_access_token_url(code: str) -> str:
    r = PreparedRequest()
    r.prepare(
        url="https://github.com/login/oauth/access_token",
        params={
            "client_id": client_id,
            "client_secret": client_secret,
            "code": code,
        },
    )
    url = r.url
    assert isinstance(url, str)
    return url


@app.route("/")
def home():
    if "username" not in session:
        return render_template(
            "anon.html", github_url=log_in_with_github_url()
        )
    return render_template(
        "success.html",
        avatar_url=session["avatar"],
        username=session["username"],
    )


@app.route("/logout")
def logout():
    session.clear()
    return redirect("/")


@app.route("/github/auth/redirect")
def github_redirect():
    if request.args.get("error"):
        return render_template("failure.html")
    code = request.args["code"]
    access_token = obtain_access_token(code)
    user_details = get_user_details(access_token)
    session["username"] = user_details.name
    session["avatar"] = user_details.avatar
    return redirect("/")


def obtain_access_token(code: str) -> str:
    url = github_access_token_url(code)
    return requests.post(url, headers={"Accept": "application/json"}).json()[
        "access_token"
    ]


class UserDetails:
    def __init__(self, name: str, avatar_url: str):
        self.name = name
        self.avatar = avatar_url


def get_user_details(bearer: str) -> UserDetails:
    j = requests.get(
        "https://api.github.com/user",
        headers={"Authorization": f"token {bearer}"},
    ).json()
    return UserDetails(j["login"], j["avatar_url"])


if __name__ == "__main__":
    app.run(debug=False, port=PORT)
