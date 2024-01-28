from flask import abort
from sqlalchemy import and_, func

from models import Record, record_schema, record_count_schema
from datetime import datetime

from config import db

def get(sort: str = None, limit: int = None, start: datetime = None, end: datetime = None):
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


def aggregated():
    c = (
        db.session
        .query(Record.rollercoaster_id, func.count(Record.rollercoaster_id).label('count'))
        .group_by(Record.rollercoaster_id)
        .order_by('count DESC')
    )

    return record_count_schema.dump(c)
