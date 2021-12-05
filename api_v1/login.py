from fastapi import APIRouter


from fastapi import Body, status, HTTPException
from fastapi.responses import Response, JSONResponse
from typing import Optional
from pydantic import EmailStr, BaseModel
from core import settings

from schemas.user import CreateUser
from core.db import get_db
from fastapi import Depends
from fastapi.security import OAuth2PasswordRequestForm
from sqlalchemy.orm import Session
from cruds import crud_user
from passlib.hash import argon2 # type: ignore
from core.security import decode
from schemas import AccessToken
from fastapi.encoders import jsonable_encoder
from core.security import generate_access_token

router = APIRouter()


class LoginSchema(BaseModel):
    email: EmailStr
    password: str


@router.post("")
async def login(db: Session = Depends(get_db), *, login_data: OAuth2PasswordRequestForm = Depends()):
    user = crud_user.get_by_email(db, email=login_data.username)

    if not user:
        return JSONResponse(
            content={"message": "Invalid account details!"},
            status_code=status.HTTP_404_NOT_FOUND,
        )

    is_password_correct = argon2.verify(login_data.password, user.hashed_password)

    if is_password_correct:
        access_token = await generate_access_token(user)
        return JSONResponse(
                content={"access_token": access_token, "token_type":"Bearer"}, status_code=status.HTTP_200_OK
        )
    else:
        return JSONResponse(
            content={"message": "Password incorrect!"},
            status_code=status.HTTP_401_UNAUTHORIZED,
        )
