from sqlalchemy import (
    ARRAY,
    BigInteger,
    Boolean,
    Column,
    DateTime,
    Enum,
    ForeignKey,
    Integer,
    SmallInteger,
    String,
)
from sqlalchemy.ext.hybrid import hybrid_property
from sqlalchemy.orm import relationship
from sqlalchemy.sql import func
from sqlalchemy.dialects import postgresql
from uuid import uuid4

from core import UserType
from core.db import Base


# TODO: Composite index for fname, mname and lname
class User(Base):
    id = Column(Integer, primary_key=True, index=True)
    uuid = Column(postgresql.UUID(as_uuid=True), default=uuid4, index=True)

    first_name = Column(String, index=True)
    middle_name = Column(String, index=False, nullable=True)
    last_name = Column(String, index=True)

    email = Column(String, index=True, nullable=False, unique=True)

    phone = Column(BigInteger, index=True, nullable=True)  # TODO: Validation

    two_fa_secret = Column(String, nullable=True, default=None)

    hashed_password = Column(String, nullable=False)

    is_verified = Column(Boolean(), default=False)

    is_active = Column(Boolean(), default=True)

    user_type = Column(
        Enum(
            UserType, values_callable=lambda x: [str(e.value) for e in UserType]
        ),  # To store enum value instead of key in db
        default=UserType.USER,
        nullable=False,
        index=True,
    )

    extras = relationship("UserExtras", back_populates="user")

    access_zones = Column(ARRAY(SmallInteger), nullable=False, default=[1])

    created_on = Column(DateTime, nullable=False, server_default=func.now())

    @hybrid_property
    def full_name(self):
        if self.middle_name:
            return f"{self.first_name} {self.middle_name} {self.last_name}"
        else:
            return f"{self.first_name} {self.last_name}"

    __tablename__ = "user"


class UserExtras(Base):
    id = Column(Integer, ForeignKey("user.id"), primary_key=True)
    user = relationship("User", back_populates="extras")

    profile_image = Column(String(100))
