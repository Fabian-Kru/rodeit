from flask import abort
from sqlalchemy import and_

from models import Record, record_schema
from datetime import datetime


def get(token_info = True, sort: str = None, limit: int = None, start: datetime = None, end: datetime = None):
    if not token_info:
        abort(401, "Not authorized")

    match sort:
        case "id":
            key = Record.id
        case "user_id":
            key = Record.user_id
        case "rollercoaster_id":
            key = Record.rollercoaster_id
        case "timestamp":
            key = Record.timestamp
        case _:
            key = None

    condition = True
    if start is not None and end is not None:
        condition = and_(Record.timestamp >= start, Record.timestamp <= end)
    elif start is not None:
        condition = Record.timestamp >= start
    elif end is not None:
        condition = Record.timestamp <= end

    c = Record.query.order_by(key).limit(limit).filter(condition).all()

    return record_schema.dump(c)