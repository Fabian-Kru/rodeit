from flask import abort
from models import Credit, credit_schema


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
