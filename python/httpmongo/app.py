import os

from flask import Flask, redirect, request
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
            {"term": i["term"], "decription": i["description"]}
            for i in self.collection.find()
        ]


repo = Repository()


@app.route("/terms")
def get_all():
    return {"all": repo.get_all()}


@app.route("/")
def form():
    return """<html>
<body>
<a href="terms">show all</a>
<form action="term" method="POST">
<input placeholder="term" name="term">
<input placeholder="description" name="description">
<button>submit</button>
"""


@app.route("/term", methods=["POST"])
def insert_one():
    repo.insert(
        {
            "term": request.form["term"],
            "description": request.form["description"],
        }
    )
    return redirect("/")
