from jose import jwt, JWTError
import config


def decode_token(token):
    try:
        return jwt.decode(token, config.JWT_TOKEN, algorithms=["HS256"])
    except JWTError as e:
        return None
    return None
