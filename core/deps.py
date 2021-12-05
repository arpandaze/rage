from fastapi import Depends, HTTPException, status
from fastapi.security import OAuth2, OAuth2PasswordBearer
from core.db import get_db

from core import TokenType

from schemas import AccessToken
from core.db import get_db
from fastapi import Depends
from sqlalchemy.orm import Session
from core.security import decode
from schemas import AccessToken
from core.security import decode
from cruds import crud_user
from core import settings

oauth2_scheme = OAuth2PasswordBearer(tokenUrl=f"{settings.API_BASE_URL}/auth")


async def user_extractor(
    db: Session = Depends(get_db),
    auth_token: str = Depends(oauth2_scheme),
):
    try:
        serialized_token = AccessToken(**decode(auth_token))
    except Exception:
        raise HTTPException(status_code=status.HTTP_401_UNAUTHORIZED)

    assert serialized_token.rid == TokenType.AccessToken  # type: ignore

    uuid = serialized_token.sub  # type: ignore

    user = crud_user.get_by_uuid(db, uuid)

    if not user.is_active:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN, detail="Account is inactive!"
        )

    if not user.is_verified:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN, detail="Account is not verified!"
        )

    return user
