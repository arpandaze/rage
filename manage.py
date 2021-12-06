from dotenv import load_dotenv
import click

load_dotenv(".env_dev")


@click.group()
def main():
    pass


class Commands:
    def settings(self):
        from core import settings

        settings_dict = settings.dict()

        for key in settings_dict.keys():
            value = settings_dict[key]

            if type(value) == list:
                value = [str(item) for item in value]

            print(f"{key}={value}")

    def start(self):
        from main import run

        run()

    def mkmig(self):
        from alembic import command
        from alembic.config import Config

        alembic_cfg = Config("alembic.ini")
        msg = input("Enter a message: ")
        command.revision(config=alembic_cfg, autogenerate=True, message=msg)
        click.echo("Inside migrate")

    def mig(self):
        from alembic import command
        from alembic.config import Config

        alembic_cfg = Config("alembic.ini")
        command.upgrade(alembic_cfg, "head")

    def cleanmig(self):
        import os

        for file in os.listdir("alembic/versions/"):
            if file != ".gitkeep":
                if os.path.isfile(f"alembic/versions/{file}"):
                    os.remove(f"alembic/versions/{file}")

    def cleandb(self):
        try:
            from alembic import command
            from alembic.config import Config

            from core.db import engine

            self.cleanmig()
            engine.execute("DROP schema public CASCADE")
            engine.execute("CREATE schema public")
            alembic_cfg = Config("alembic.ini")
            command.revision(
                config=alembic_cfg,
                autogenerate=True,
                message="Autogenerated by cleandb()",
            )
            command.upgrade(alembic_cfg, "head")
        except Exception as e:
            print(e)

    def pytest(self):
        import os

        os.system("python -m pytest --verbose --color=yes tests/")


commands = Commands()


@click.command()
def cleanmig():
    commands.cleanmig()


@click.command()
def mig():
    commands.mig()


@click.command()
def mkmig():
    commands.mkmig()


@click.command()
def cleandb():
    commands.cleandb()


@click.command()
def start():
    commands.start()


@click.command()
def settings():
    commands.settings()


@click.command()
def pytest():
    commands.pytest()


command_list = list(filter(lambda x: x if x[0] != "_" else None, commands.__dir__()))

for command in command_list:
    main.add_command(eval(f"{command}"))


if __name__ == "__main__":
    main()