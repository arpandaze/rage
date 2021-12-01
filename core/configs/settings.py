import base64
import secrets
from typing import TYPE_CHECKING, Any, Dict, List, Optional, Union

from pydantic import (
    AnyHttpUrl,
    BaseSettings,
    EmailStr,
    PostgresDsn,
    RedisDsn,
    validator,
)

if TYPE_CHECKING:
    AnyHttpUrl = str


class Settings(BaseSettings):
    PROJECT_NAME: str = "MyAwesomeProject"
    MODE: str = "DEV"

    SECRET: str = secrets.token_urlsafe(32)
    SECRET_KEY: Optional[bytes] = None

    @validator("SECRET_KEY", pre=True)
    def generate_secret_key(cls, v: Optional[str], values: Dict[str, Any]) -> bytes:
        secret = values["SECRET"]

        padding = 4 - (len(secret) % 4)
        secret = secret + ("=" * padding)

        return base64.urlsafe_b64decode(secret)

    ENCRYPT_TOKEN: bool = True

    @validator("ENCRYPT_TOKEN")
    def encrypt_token(cls, v: Optional[str]) -> bool:
        if str(v) == "True":
            return True
        elif str(v) == "False":
            return False
        else:
            raise ValueError(f"Expected `True` or `False`! Got {v}")

    PROTOCOL: str = "http"

    API_V1_PREFIX: str = "/api/v1"

    TOTP_WINDOW: int = 1

    BACKEND_CORS_ORIGINS: List[AnyHttpUrl] = [
        "http://localhost:3001",
        "http://localhost",
    ]

    @validator("BACKEND_CORS_ORIGINS", pre=True)
    def assemble_cors_origins(cls, v: Union[str, List[str]]) -> Union[List[str], str]:
        if isinstance(v, str) and not v.startswith("["):
            return [i.strip() for i in v.split(",")]
        elif isinstance(v, (list, str)):
            return v
        raise ValueError(v)

    POSTGRES_HOST: str = "localhost"
    POSTGRES_PORT: str = "5432"
    POSTGRES_USER: str = "user"
    POSTGRES_PASSWORD: str = "password"
    POSTGRES_DB: str = "app"
    POSTGRES_URI: Optional[PostgresDsn] = None

    @validator("POSTGRES_URI", pre=True)
    def construct_postgres_uri(cls, v: Optional[str], values: Dict[str, Any]) -> Any:
        if isinstance(v, str):
            return v
        return PostgresDsn.build(
            scheme="postgresql",
            user=values["POSTGRES_USER"],
            password=values["POSTGRES_PASSWORD"],
            host=values["POSTGRES_HOST"],
            path=f"/{values['POSTGRES_DB'] or ''}",
        )

    REDIS_HOST: str = "localhost"
    REDIS_USER: str = "user"
    REDIS_PASSWORD: str = "password"
    REDIS_URI: Optional[RedisDsn] = None

    @validator("REDIS_URI", pre=True)
    def construct_redis_uri(cls, v: Optional[str], values: Dict[str, Any]) -> Any:
        if isinstance(v, str):
            return v
        return RedisDsn.build(
            scheme="redis",
            user=values["REDIS_USER"],
            password=values["REDIS_PASSWORD"],
            host=values["REDIS_HOST"],
        )

    SUPERADMIN_EMAIL: EmailStr = "superadmin@myawesomeproject.local"  # type: ignore
    SUPERADMIN_PHONE: Optional[int] = None
    SUPERADMIN_PASSWORD: str = "password"

    UVICORN_HOST: str = "localhost"
    UVICORN_PORT: int = 8080
    UVICORN_WORKERS: int = 1

    API_BASE_URL: Optional[AnyHttpUrl] = None
    SERVER_FRONTEND_URL: str = "http://localhost:8080"

    @validator("API_BASE_URL", pre=True)
    def construct_base_url(cls, v: Optional[str], values: Dict[str, Any]) -> Any:
        if isinstance(v, AnyHttpUrl):
            return v
        return AnyHttpUrl.build(  # type: ignore
            scheme=values["PROTOCOL"],
            host=values["UVICORN_HOST"],
            port=str(values["UVICORN_PORT"]),
            path=values["API_V1_PREFIX"],
        )

    SMTP_TLS: bool = True

    @validator("SMTP_TLS")
    def smtp_tls(cls, v: Optional[str]) -> bool:
        if str(v) == "True":
            return True
        elif str(v) == "False":
            return False
        else:
            raise ValueError(f"Expected `True` or `False`! Got {v}")

    SMTP_HOST: str = "localhost"
    SMTP_PORT: int = 1025
    SMTP_USER: Optional[str] = None
    SMTP_PASSWORD: Optional[str] = None
    EMAILS_FROM_EMAIL: EmailStr = "noreply@myawesomeproject.local"  # type: ignore
    EMAILS_FROM_NAME: Optional[str] = None

    @validator("EMAILS_FROM_NAME")
    def get_project_name(cls, v: Optional[str], values: Dict[str, Any]) -> str:
        if not v:
            return values["PROJECT_NAME"]
        return v

    EMAIL_VERIFICATION_TIMEOUT: int = 86400 # seconds

    class Config:
        case_sensitive = True


settings = Settings()
