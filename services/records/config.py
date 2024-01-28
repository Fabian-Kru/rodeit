# adapted from https://realpython.com/flask-connexion-rest-api-part-2/

import pathlib
import connexion

from os import environ
from flask_sqlalchemy import SQLAlchemy
from flask_marshmallow import Marshmallow


JWT_TOKEN = environ['JWT_TOKEN']

basedir = pathlib.Path(__file__).parent.resolve()
connex_app = connexion.App(__name__, specification_dir=basedir)

app = connex_app.app
app.config["SQLALCHEMY_DATABASE_URI"] = f"sqlite:///{basedir / 'db/records.db'}"
app.config["SQLALCHEMY_TRACK_MODIFICATIONS"] = False

db = SQLAlchemy(app)
marshmallow = Marshmallow(app)

with app.app_context():
    import models
    db.create_all()
