from jose import jwt, JWTError


def decode_token(token):
    try:
        return jwt.decode(token, "public_key", algorithms=["HS256"])
    except JWTError as e:
        return None
    return None
