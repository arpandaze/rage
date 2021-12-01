from jose import jwt, jws, jwe
from fastapi.encoders import jsonable_encoder
from schemas import VerificationToken
from core import settings
import time
import json

data = VerificationToken(iat=time.time(), exp=1638263289, sub=1)


def encrypt(data):
    data_to_encode = jsonable_encoder(data)
    jw_token = jwt.encode(data_to_encode, settings.SECRET_KEY, algorithm="HS256")

    if settings.ENCRYPT_TOKEN:
        return jwe.encrypt(
            jw_token, settings.SECRET_KEY, algorithm="dir", encryption="A256GCM"
        )
    else:
        return jwt_token


def decode(token):
    if settings.ENCRYPT_TOKEN:
        token = jwe.decrypt(token, settings.SECRET_KEY)

    return jwt.decode(jw_token, settings.SECRET_KEY, algorithms="HS256")


enc = encode(data)
print(f"Encoded: {enc}")
dec = decode(enc)
print(f"Decoded: {dec}")
