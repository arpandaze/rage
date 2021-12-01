from typing import Any, Dict

import emails
from emails.template import JinjaTemplate

from core import settings
from core.security import generate_email_verification_token


async def send_email(
    email_to: str,
    subject_template: str = "",
    html_template: str = "",
    environment: Dict[str, Any] = {},
) -> None:
    message = emails.Message(
        subject=JinjaTemplate(subject_template),
        html=JinjaTemplate(html_template),
        mail_from=(settings.EMAILS_FROM_NAME, settings.EMAILS_FROM_EMAIL),
    )
    smtp_options = {"host": settings.SMTP_HOST, "port": settings.SMTP_PORT}
    if settings.SMTP_TLS:
        smtp_options["tls"] = True
    if settings.SMTP_USER:
        smtp_options["user"] = settings.SMTP_USER
    if settings.SMTP_PASSWORD:
        smtp_options["password"] = settings.SMTP_PASSWORD

    return message.send(to=email_to, render=environment, smtp=smtp_options)


async def send_verification_email(user) -> None:
    project_name = settings.PROJECT_NAME
    subject = f"{project_name} - Verification Email"

    with open("utils/templates/verification-email.html") as f:
        template_str = f.read()

    verification_token = await generate_email_verification_token(user)
    print(type(verification_token))

    server_host = settings.SERVER_FRONTEND_URL

    link = f"{server_host}/verify/?token={verification_token}"
    await send_email(
        email_to=user.email,
        subject_template=subject,
        html_template=template_str,
        environment={
            "name": user.full_name,
            "link": link,
        },
    )
