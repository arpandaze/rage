from authlib.jose import JsonWebEncryption
from authlib.jose.errors import JoseError
from core import settings
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives.kdf.pbkdf2 import PBKDF2HMAC
import secrets
from pydantic import BaseModel
import time
from enum import Enum
from schemas import VerificationToken, AccessToken
from core import TokenType
from fastapi.encoders import jsonable_encoder
import json
from jose import jwe, jwt
from jose.constants import ALGORITHMS

# jwe = JsonWebEncryption()
protected = {"alg": "A256KW", "enc": "A256CBC-HS512"}


def encode(data):
    data_to_encode = jsonable_encoder(data)
    jw_token = jwt.encode(data_to_encode, settings.SECRET_KEY, algorithm="HS256")

    if settings.ENCRYPT_TOKEN:
        return jwe.encrypt(
            jw_token, settings.SECRET_KEY, algorithm="dir", encryption="A256GCM"
        ).decode("UTF-8")
    else:
        return jw_token


def decode(token):
    if settings.ENCRYPT_TOKEN:
        token = jwe.decrypt(token, settings.SECRET_KEY)

    return jwt.decode(token, settings.SECRET_KEY, algorithms="HS256")


async def generate_email_verification_token(user):
    current_timestamp = int(time.time())

    payload = VerificationToken(
        iat=current_timestamp,
        exp=current_timestamp + settings.EMAIL_VERIFICATION_TIMEOUT,
        sub=str(user.uuid),
    )

    return encode(payload)


async def generate_access_token(user):
    current_timestamp = int(time.time())

    payload = AccessToken(
        iat=current_timestamp,
        exp=current_timestamp + settings.ACCESS_TOKEN_TIMEOUT,
        sub=str(user.uuid),
    )

    return encode(payload)
