import sys
import os
import platform

system = platform.system()


def main(args):
    if len(args) == 1:
        print("No args provided!")
        print("Available args: dk")
        return None

    if args[1] == "dk":
        if system == "Darwin":
            os.system("mailhog > /dev/null 2>&1 &")
            os.system(
                "redis-server --appendonly yes --requirepass redispass > /dev/null 2>&1 &"
            )
            os.system(
                "export DATABASE_URL=postgres://postuser:postpass@localhost:5432/actix && sqlx database create && sqlx migrate run"
            )
        else:
            os.system("./scripts/redis.sh")
            os.system("./scripts/db.sh")

    elif args[1] == "start":
        os.system("cargo watch -x 'run --bin rage'")

    elif args[1] == "mig":
        os.system(
            "DATABASE_URL=postgres://postuser:postpass@localhost:5432/actix cargo sqlx prepare --merged -- --all-targets --all-features"
        )

    elif args[1] == "migtest":
        os.system(
            "DATABASE_URL=postgres://postuser:postpass@localhost:5432/actix cargo sqlx prepare -- --tests --bin rage"
        )


if __name__ == "__main__":
    main(sys.argv)
