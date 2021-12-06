from enum import Enum

class TokenType(Enum):
    RefreshToken = 0
    AccessToken = 1
    VerificationToken = 2
    EmailChangeToken = 3
    TOTPAccessToken = 4
