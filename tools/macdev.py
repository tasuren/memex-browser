import subprocess
from argparse import ArgumentParser
from typing import Final

from _lib import setup_logger

logger: Final = setup_logger("macdebug")


def cargo_build(package: str, *, release: bool) -> None:
    subprocess.run(
        ("cargo", "build", "-p", package) + (("--release",) if release else ()),
        check=True,
    )


def macbundle(*, release: bool) -> None:
    subprocess.run(
        ("uv", "run", "tools/macbundle.py") + (("--release",) if release else ()),
        check=True,
    )


def run(*, is_release: bool, lldb: bool) -> None:
    product_name = "Memex Browser" if is_release else "Memex Browser (D)"

    cmd = f"./target/debug/{product_name}.app/Contents/MacOS/{product_name}"
    if lldb:
        cmd = ("lldb", cmd)

    subprocess.run(cmd, check=True)


parser = ArgumentParser(prog="macdebug")

parser.add_argument("--release", action="store_true")
parser.add_argument("-b", "--build", action="store_true")
parser.add_argument("-H", "--build-helper", action="store_true")
parser.add_argument("-B", "--bundle", action="store_true")
parser.add_argument("-r", "--run", action="store_true")
parser.add_argument("-d", "--lldb", action="store_true")

args = parser.parse_args()

try:
    if args.build:
        logger.info("memex-appのビルド")
        cargo_build("memex-app", release=args.release)

    if args.build_helper:
        logger.info("memex-cef-helperのビルド")
        cargo_build("memex-cef-helper", release=args.release)

    if args.bundle:
        logger.info("バンドルの作成")
        macbundle(release=args.release)

    if args.run:
        logger.info("=== バンドルの実行 ===\n")
        run(is_release=args.release, lldb=args.lldb)
except KeyboardInterrupt:
    exit(130)
except subprocess.CalledProcessError as e:
    logger.error("コマンドの実行がエラーで終了しました: %s", str(e))
