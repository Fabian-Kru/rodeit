import json

from flask import abort
from models import Record, record_schema
from config import db


def main():
    return ""


def get_all():
    c = Record.query.all()
    return record_schema.dump(c)


def get_by_id(record_id: int):
    res = Record.query.filter(Record.id == record_id).one_or_none()
    if res is not None:
        return record_schema.dump(res)
    else:
        abort(404, f"Not found")


def get(user_id: int):
    res = Record.query.filter(Record.user_id == user_id).one_or_none()
    if res is not None:
        return record_schema.dump(res)
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
    entry = record_schema.loads(json.dumps(new_entry))
    db.session.add(entry)
    db.session.commit()
    return record_schema.dump(entry), 201
