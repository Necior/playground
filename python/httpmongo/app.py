from itertools import groupby
import os

from flask import escape, Flask, redirect, request
import pymongo


app = Flask(__name__)


class Repository:
    def __init__(self):
        host = os.environ.get("HTTPMONGO_MONGODB_HOST", "localhost")
        self.collection = pymongo.MongoClient(
            host, serverSelectionTimeoutMS=2000
        ).httpmongodb.httpmongocollection

    def insert(self, doc):
        self.collection.insert_one(doc)

    def get_all(self):
        return [
            {"term": i["term"], "description": i["description"]}
            for i in self.collection.find()
        ]


repo = Repository()


def view_terms():
    key = lambda t: t["term"]

    html = ""
    for key, group in groupby(sorted(repo.get_all(), key=key), key=key):
        html += f"<h4>{escape(key)}</h4>"
        for term in group:
            desc = term["description"]
            html += "<ul>"
            html += f"<li>{escape(desc)}</li>"
            html += "</ul>"
    html = html or "no terms yet"
    return html


@app.route("/")
def form():
    return (
        """<!DOCTYPE html>
<html lang="en">
<head><title>Terms</title></head>
<body>
<form action="term" method="POST">
<input placeholder="term" name="term">
<input placeholder="description" name="description">
<button>submit</button>
</form>
<hr>
"""
        + view_terms()
    )


@app.route("/term", methods=["POST"])
def insert_one():
    repo.insert(
        {
            "term": request.form["term"].strip(),
            "description": request.form["description"].strip(),
        }
    )
    return redirect("/")
