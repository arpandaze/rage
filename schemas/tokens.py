from pydantic import BaseModel
from core import TokenType

class TokenBase(BaseModel):
    iat: int
    exp: int
    sub: str
    rid: TokenType


class VerificationToken(TokenBase):
    rid: TokenType = TokenType.VerificationToken
