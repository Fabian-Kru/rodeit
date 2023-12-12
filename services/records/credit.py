import json

from flask import abort
from models import Credit, credit_schema, credit_schema_single
from config import db


def main():
    return ""


def get_all():
    c = Credit.query.all()
    return credit_schema.dump(c)


def get_by_id(credit_id: int):
    res = Credit.query.filter(Credit.id == credit_id).one_or_none()
    if res is not None:
        return credit_schema.dump(res)
    else:
        abort(404, f"Not found")


def get(user_id: int):
    res = Credit.query.filter(Credit.user_id == user_id).one_or_none()
    if res is not None:
        return credit_schema.dump(res)
    else:
        abort(404, f"Not found")


# example:
# curl -X 'POST' 'http://0.0.0.0:8000/submit' -H 'Content-Type: application/json' -d '{"user_id": 1, "rollercoaster_id" : 1}'
def submit(body: dict):
    user_id = body.get("user_id")
    rollercoaster_id = body.get("rollercoaster_id")
    new_entry = {
        "user_id": str(user_id),
        "rollercoaster_id": str(rollercoaster_id),
    }
    entry = credit_schema_single.loads(json.dumps(new_entry))
    db.session.add(entry)
    db.session.commit()
    return credit_schema.dump(entry), 201
