from sqlalchemy.orm import Session
from typing import Optional

from core import CRUDBase
from models import User
from schemas import CreateUser, UpdateUser


class CRUDUser(CRUDBase[User, CreateUser, UpdateUser]):
    def get_by_email(self, db: Session, email: str) -> Optional[User]:
        return db.query(User).filter_by(email=email).first()

    def get_by_phone(self, db: Session, phone: int):
        return db.query(User).filter_by(phone=phone).first()

    def get_by_uuid(self, db: Session, uuid: str):
        return db.query(User).filter_by(uuid=uuid).first()

    def verify_user(self, db: Session, user: User):
        self.update(db, db_obj=user, obj_in={"is_verified": True})


crud_user = CRUDUser(User)
