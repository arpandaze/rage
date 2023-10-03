#!/usr/bin/env bash
set -x
set -eo pipefail

if [ "$(podman ps -q -f name=actix_mailpit)" ]; then
    echo "Mailpit already running!"
    exit
fi

if [ "$(podman ps -aq -f name=actix_mailpit)" ]; then
    echo "Launching existing mailpit container!"
    podman start actix_mailpit
    exit
fi

echo "Creating new mailpit container!"

podman run --name actix_mailpit \
  --restart unless-stopped \
  -e MP_SMTP_AUTH_ALLOW_INSECURE=true \
  -e MP_SMTP_AUTH_ACCEPT_ANY=true \
  -p 8025:8025 \
  -p 1025:1025 \
  -d axllent/mailpit
