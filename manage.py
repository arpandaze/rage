import sys
import os

def main(args):
    if len(args) == 1:
        print("No args provided!")
        print("Available args: dk")
        return None

    if args[1] == "dk":
        os.system("./scripts/redis.sh")
        os.system("./scripts/db.sh")

    elif args[1] == "start":
        os.system("cargo watch -x 'run --bin actix-backend'")

    elif args[1] == "mig":
        os.system("export DATABASE_URL=postgres://postuser:postpass@localhost:5432/actix && cargo sqlx prepare -- --bin actix-backend")
     

if __name__ == "__main__":
    main(sys.argv)
