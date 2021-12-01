from fastapi import APIRouter, Body, status, HTTPException
from fastapi.responses import Response
from typing import Optional
from pydantic import EmailStr, BaseModel

from schemas.user import CreateUser
from core.db import get_db
from fastapi import Depends
from sqlalchemy.orm import Session
from cruds import crud_user
from passlib.hash import argon2  # type: ignore
from utils.email import send_verification_email

router = APIRouter()


class RegistrationForm(BaseModel):
    first_name: str
    middle_name: Optional[str]
    last_name: str
    email: EmailStr
    phone: Optional[int]
    password: str


@router.post("/register")
async def register(
    db: Session = Depends(get_db), *, form_data: RegistrationForm = Body(...)
):
    existing_user = crud_user.get_by_email(db, form_data.email)

    if existing_user:
        raise HTTPException(
            status_code=status.HTTP_409_CONFLICT,
            detail="User with the email already exists",
        )

    hashed_password = argon2.hash(form_data.password)

    create_details = CreateUser(
        first_name=form_data.first_name,
        middle_name=form_data.middle_name,
        last_name=form_data.last_name,
        email=form_data.email,
        phone=form_data.phone,
        hashed_password=hashed_password,
    )

    user = crud_user.create(db, obj_in=create_details)

    await send_verification_email(user)

    return Response(status_code=status.HTTP_201_CREATED)
