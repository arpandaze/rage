from fastapi import APIRouter


from fastapi import Body, status, HTTPException
from fastapi.responses import Response
from typing import Optional
from pydantic import EmailStr, BaseModel
from core import settings

from schemas.user import CreateUser
from core.db import get_db
from fastapi import Depends
from sqlalchemy.orm import Session
from cruds import crud_user
from passlib.hash import argon2  # type: ignore
from core.security import decode
from schemas import VerificationToken
from fastapi.encoders import jsonable_encoder

router = APIRouter()


class VerificationData(BaseModel):
    token: str


@router.post("/verify/email")
def verify_email(
    db: Session = Depends(get_db), *, verification_data: VerificationData = Body(...)
):
    token = VerificationToken(**decode(verification_data.token))

    user = crud_user.get_by_uuid(db, uuid=token.sub)
    crud_user.verify_user(db, user)

    return Response(status_code=status.HTTP_201_CREATED)


@router.post("/verify/phone")
def verify_phone(
    db: Session = Depends(get_db), *, verification_data: VerificationData = Body(...)
):
    pass
    return Response(status_code=status.HTTP_201_CREATED)
