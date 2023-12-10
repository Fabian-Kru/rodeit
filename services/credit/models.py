from datetime import datetime
from config import db, marshmallow


class Credit(db.Model):
    __tablename__ = "credit"
    id = db.Column(db.Integer, primary_key=True, unique=True)
    user_id = db.Column(db.String(32))
    rollercoaster_id = db.Column(db.Integer)
    timestamp = db.Column(
        db.DateTime, default=datetime.utcnow, onupdate=datetime.utcnow
    )


class CreditSchema(marshmallow.SQLAlchemyAutoSchema):
    class Meta:
        model = Credit
        load_instance = True
        sqla_session = db.session


credit_schema = CreditSchema(many=True)
