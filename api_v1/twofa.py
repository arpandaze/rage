from fastapi import APIRouter, Header, Form
from fastapi.encoders import jsonable_encoder


from fastapi import Body, status, HTTPException
from fastapi.responses import Response, JSONResponse
from fastapi.security import OAuth2
from typing import Optional
from pydantic import EmailStr, BaseModel
from pydantic.errors import JsonError
from core import settings

from schemas.user import CreateUser
from core.db import get_db
from fastapi import Depends
from fastapi.security import OAuth2PasswordRequestForm
from sqlalchemy.orm import Session
from cruds import crud_user
from passlib.hash import argon2  # type: ignore
from core.security import decode
from schemas import Token
from fastapi.encoders import jsonable_encoder
from core.security import generate_access_token
from core.deps import bearer_token
from core import TokenType
from pyotp import TOTP

router = APIRouter()



@router.post("/verify")
async def twofa_verify(
    db: Session = Depends(get_db),
    *,
    auth_token: str = Depends(bearer_token),
    totp_code: str = Form(...)
):
    token_data = Token(**decode(auth_token))

    assert token_data.rid == TokenType.TOTPAccessToken  # type:ignore

    user = crud_user.get_by_uuid(db, uuid=token_data.sub)

    totp = TOTP(user.two_fa_secret)

    if totp.verify(totp_code, valid_window=settings.TOTP_WINDOW):
        access_token = await generate_access_token(user)
        return JSONResponse(
            content={"access_token": access_token, "token_type": "Bearer"},
            status_code=status.HTTP_200_OK,
        )
    else:
        return JSONResponse(status_code=status.HTTP_401_UNAUTHORIZED, content={"message":"Invalid TOTP token!"})
