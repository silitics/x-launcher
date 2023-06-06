import time

import click


def run_build():
    print("Building the project takes some time...")
    time.sleep(10)
    print("Done!")


def run_test():
    run_build()
    print("Testing takes even more time...")
    time.sleep(20)
    print("Done!")


@click.group()
def main():
    pass


@main.command()
def build():
    """Builds the project."""
    run_build()


@main.command()
def test():
    """Tests the project."""
    run_test()
