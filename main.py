import os

import uvicorn
from fastapi import FastAPI
from fastapi.openapi.docs import (
    get_swagger_ui_html,
    get_swagger_ui_oauth2_redirect_html,
)
from starlette.middleware.cors import CORSMiddleware

from api_v1 import router
from core import settings
from core.db import SessionLocal
from cruds import crud_user
from models import User
from schemas import CreateUser

app = FastAPI(
    title=settings.PROJECT_NAME,
    openapi_url=f"{settings.API_V1_PREFIX}/openapi.json",
    docs_url=None,
)

@app.on_event("startup")
async def startup():
    db = SessionLocal()
    super_admin = db.query(User).filter_by(email=settings.SUPERADMIN_EMAIL)
    if not super_admin:
        user_in = CreateUser(
            first_name="Super",
            last_name="Admin",
            email=settings.SUPERADMIN_EMAIL,
            phone=settings.SUPERADMIN_PHONE,
            password=settings.SUPERADMIN_PASSWORD,
        )
        super_admin = crud_user.create(db, obj_in=user_in)  # noqa: F841


@app.on_event("shutdown")
async def shutdown():
    pass


@app.get("/docs", include_in_schema=False)
async def custom_swagger_ui_html():
    return get_swagger_ui_html(
        openapi_url=app.openapi_url,
        title=app.title + " - API Documentaion",
        oauth2_redirect_url=app.swagger_ui_oauth2_redirect_url,
        swagger_js_url="/core/api/v1/utils/static/swagger-ui-bundle.js",
        swagger_css_url="/core/api/v1/utils/static/swagger-ui.css",
    )


@app.get(app.swagger_ui_oauth2_redirect_url, include_in_schema=False)
async def swagger_ui_redirect():
    return get_swagger_ui_oauth2_redirect_html()


if settings.BACKEND_CORS_ORIGINS:
    app.add_middleware(
        CORSMiddleware,
        allow_origins=[str(origin) for origin in settings.BACKEND_CORS_ORIGINS],
        allow_credentials=True,
        allow_methods=["*"],
        allow_headers=["*"],
    )
    pass

app.include_router(router, prefix=settings.API_V1_PREFIX)


def run():
    reload_blacklist = ["tests", ".pytest_cache"]
    reload_dirs = os.listdir()
    for dir in reload_blacklist:
        try:
            reload_dirs.remove(dir)
        except Exception:
            pass
    uvicorn.run(
        "main:app",
        host=settings.UVICORN_HOST,
        port=settings.UVICORN_PORT,
        reload=True if settings.MODE == "dev" else False,
        reload_dirs=reload_dirs,
        debug=True if settings.MODE == "dev" else False,
        workers=settings.UVICORN_WORKERS,
    )


if __name__ == "__main__":
    run()
