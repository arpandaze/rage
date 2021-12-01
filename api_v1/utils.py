from fastapi import APIRouter, Body
from fastapi.responses import FileResponse, Response
from core import settings
from fastapi.encoders import jsonable_encoder
from pydantic import BaseModel
import os
import requests

router = APIRouter()


@router.get("/ping")
async def ping():
    return "pong"


class Token(BaseModel):
    user_id: int
    name: str
    email: str


@router.post("/encode")
async def encode(data: Token = Body(...)):
    pass

@router.get("/static/swagger-ui-bundle.js")
async def swagger_js():
    swagger_js_path = ".local/swagger-ui-bundle.js"

    if not os.path.exists(swagger_js_path):
        if not os.path.isdir(".local"):
            os.mkdir(".local")

        swagger_js_req = requests.get("https://cdn.jsdelivr.net/npm/swagger-ui-dist@3/swagger-ui-bundle.js")


        assert swagger_js_req.status_code == 200, "Couldn't find swagger-ui-bundle.js!"

        with open(".local/swagger-ui-bundle.js", "wb") as f:
            f.write(swagger_js_req.content)

    return FileResponse(".local/swagger-ui-bundle.js")




@router.get("/static/swagger-ui.css")
async def swagger_css():
    swagger_css_path = ".local/swagger-ui.css"

    if not os.path.exists(swagger_css_path):
        if not os.path.isdir(".local"):
            os.mkdir(".local")

        swagger_css_req = requests.get("https://cdn.jsdelivr.net/npm/swagger-ui-dist@3/swagger-ui.css")

        assert swagger_css_req.status_code == 200, "Couldn't find swagger-ui.css!"

        with open(".local/swagger-ui.css", "wb") as f:
            f.write(swagger_css_req.content)

    return FileResponse(".local/swagger-ui.css")
