from fastapi import APIRouter

from . import register
from . import login
from . import verify
from . import forgot
from . import twofa
from . import utils

router = APIRouter()

router.include_router(register.router, tags=["Register"])
router.include_router(login.router, prefix="/utils", tags=["Login"])
router.include_router(verify.router, prefix="/utils", tags=["Verify"])
router.include_router(forgot.router, prefix="/utils", tags=["Forgot"])
router.include_router(twofa.router, prefix="/utils", tags=["Two Factor"])
router.include_router(utils.router, prefix="/utils", tags=["Utilities"])
