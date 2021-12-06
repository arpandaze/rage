from pydantic import BaseModel
from core import TokenType

class Token(BaseModel):
    iat: int
    exp: int
    sub: str
    rid: TokenType



class VerificationToken(Token):
    rid: TokenType = TokenType.VerificationToken

class AccessToken(Token):
    rid: TokenType = TokenType.AccessToken

class TOTPAccessToken(Token):
    rid: TokenType = TokenType.TOTPAccessToken
