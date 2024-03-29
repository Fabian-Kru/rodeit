import json

from flask import abort

from models import Record, record_schema, record_schema_single
from config import db


def main():
    return ""


def get_by_id(record_id: int):
    res = Record.query.filter(Record.id == record_id).one_or_none()
    if res is not None:
        return record_schema.dump(res)
    else:
        abort(404, "Not found")


def get_by_user(user_id: int):
    res = Record.query.filter(Record.user_id == user_id).all()
    if res is not None:
        return record_schema.dump(res)
    else:
        abort(404, "Not found")


def get_by_rollercoaster(rollercoaster_id: int):
    res = Record.query.filter(Record.rollercoaster_id == rollercoaster_id).all()
    if res is not None:
        return record_schema.dump(res)
    else:
        abort(404, "Not found")


# example:
# curl -X 'POST' 'http://0.0.0.0:8000/submit' -H 'Content-Type: application/json' -d '{"user_id": 1, "rollercoaster_id" : 1}'
def submit(body: dict, token_info):
    if not token_info:
        abort(401, "Not authorized")

    if 'sub' not in token_info:
        abort(401, "Not authorized")

    user_id = str(body.get("user_id"))

    if str(token_info['sub']) != user_id:
        abort(403, "Forbidden")

    rollercoaster_id = body.get("rollercoaster_id")
    new_entry = {
        "user_id": str(user_id),
        "rollercoaster_id": str(rollercoaster_id),
    }
    entry = record_schema_single.loads(json.dumps(new_entry))
    db.session.add(entry)
    db.session.commit()
    return record_schema_single.dump(entry), 201
