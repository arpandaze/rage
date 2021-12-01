from typing import Optional

from pydantic import BaseModel, EmailStr


class CreateUser(BaseModel):
    first_name: str
    middle_name: Optional[str]
    last_name: str
    email: EmailStr
    phone: Optional[int]
    hashed_password: str


class RetrieveUser(BaseModel):
    pass


class UpdateUser(BaseModel):
    first_name: Optional[str]
    middle_name: Optional[str]
    last_name: Optional[str]
    email: Optional[EmailStr]
    alternate_email: Optional[EmailStr]
    phone: Optional[int]
    password: str
