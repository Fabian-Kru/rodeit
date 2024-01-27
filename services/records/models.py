from datetime import datetime

from marshmallow import fields

from config import db, marshmallow


class Record(db.Model):
    __tablename__ = "records"
    id = db.Column(db.Integer, primary_key=True, unique=True)
    user_id = db.Column(db.String(32))
    rollercoaster_id = db.Column(db.Integer)
    timestamp = db.Column(
        db.DateTime, default=datetime.utcnow, onupdate=datetime.utcnow
    )


class RecordSchema(marshmallow.SQLAlchemyAutoSchema):
    class Meta:
        model = Record
        load_instance = True
        sqla_session = db.session


class RecordCountSchema(marshmallow.SQLAlchemyAutoSchema):
    rollercoaster_id = fields.Integer()
    count = fields.Int(dump_only=True)


record_schema = RecordSchema(many=True)
record_schema_single = RecordSchema()
record_count_schema = RecordCountSchema(many=True)
