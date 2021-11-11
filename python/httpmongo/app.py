from itertools import groupby
import os
from typing import List

from flask import escape, Flask, redirect, request
import pymongo


app = Flask(__name__)


class Definition:
    def __init__(self, term: str, description: str):
        self.description = description
        self.term = term

    def to_dict(self) -> dict:
        return {
            "description": self.description,
            "term": self.term,
        }

    @staticmethod
    def from_dict(d: dict) -> "Definition":
        return Definition(d["term"], d["description"])


class Repository:
    def __init__(self):
        host = os.environ.get("HTTPMONGO_MONGODB_HOST", "localhost")
        self.collection = pymongo.MongoClient(
            host, serverSelectionTimeoutMS=2000
        ).httpmongodb.httpmongocollection

    def insert(self, definition: Definition) -> None:
        self.collection.insert_one(definition.to_dict())

    def get_all(self) -> List[Definition]:
        return [Definition.from_dict(d) for d in self.collection.find()]


repo = Repository()


def view_terms():
    html = ""
    for term, definitions in groupby(
        sorted(
            repo.get_all(),
            key=lambda d: (d.term.lower(), d.term, d.description),
        ),
        key=lambda d: d.term,
    ):
        html += f"<h4>{escape(term)}</h4>"
        html += "<ul>"
        for definition in definitions:
            html += f"<li><pre>{escape(definition.description)}</pre></li>"
        html += "</ul>"
    html = html or "no terms yet"
    return html


@app.route("/")
def form():
    return (
        """<!DOCTYPE html>
<html lang="en">
<head>
<title>Definitions</title>
<style>
pre {
white-space: pre-wrap;
}
</style>
</head>
<body>
<form action="term" method="POST">
<input placeholder="term" name="term" autofocus><br><br>
<textarea placeholder="description" name="description"></textarea><br><br>
<button>add definition</button>
</form>
<hr>
"""
        + view_terms()
    )


@app.route("/term", methods=["POST"])
def insert_one():
    repo.insert(
        Definition(
            request.form["term"].strip(), request.form["description"].strip()
        )
    )
    return redirect("/")
