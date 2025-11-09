__all__ = ("setup_logger",)

import logging
from os import getenv
from sys import stdout
from typing import Final


def setup_logger(name: str) -> logging.Logger:
    logger: Final = logging.getLogger(name)

    logger.setLevel(logging.DEBUG if getenv("DEBUG") == "1" else logging.INFO)
    formatter = logging.Formatter(
        "[%(asctime)s] {%(filename)s:%(lineno)d} %(levelname)s - %(message)s"
    )
    handler: Final = logging.StreamHandler(stdout)
    handler.setFormatter(formatter)
    logger.addHandler(handler)

    return logger


if __name__ == "__main__":
    raise RuntimeError(
        "このプログラムは、他のツールから使われることを目的としています。"
    )
