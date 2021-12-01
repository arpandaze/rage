from fastapi.testclient import TestClient
from fastapi import status
from core import settings

from main import app
from core.db import SessionLocal
from models import User

client = TestClient(app)
db = SessionLocal()


def test_register_with_all_details():
    user_details = {
        "first_name": "The",
        "middle_name": "Test",
        "last_name": "User",
        "email": "testuser@example.com",
        "phone": 9847111111,
        "password": "testpassword",
    }
    response = client.post(f"{settings.API_BASE_URL}/register", json=user_details)

    assert (
        response.status_code == status.HTTP_201_CREATED
    ), "User register endpoint didn't return 201"

    created_user = db.query(User).filter_by(email=user_details["email"]).first()

    assert created_user, "User not created"


def test_register_with_existing_email():
    user_details = {
        "first_name": "New",
        "middle_name": "Test",
        "last_name": "User",
        "email": "testuser@example.com",
        "phone": 9847111111,
        "password": "testpassword",
    }
    response = client.post(f"{settings.API_BASE_URL}/register", json=user_details)

    assert (
        response.status_code == status.HTTP_409_CONFLICT
    ), "Could register with existing email"

    created_user = db.query(User).filter_by(email=user_details["email"]).first()

    db.delete(created_user)
    db.commit()


def test_register_with_only_required():
    user_details = {
        "first_name": "New",
        "last_name": "User",
        "email": "testuser2@example.com",
        "password": "testpassword",
    }
    response = client.post(f"{settings.API_BASE_URL}/register", json=user_details)

    assert response.status_code == status.HTTP_201_CREATED

    created_user = db.query(User).filter_by(email=user_details["email"]).first()

    assert created_user, "User not created"

    db.delete(created_user)
    db.commit()
