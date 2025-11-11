import os
import shutil
from argparse import ArgumentParser
from os import getenv
from pathlib import PurePath
from typing import Final, cast

import tomllib
from _lib import setup_logger

logger: Final = setup_logger("macbundle")

parser = ArgumentParser()
parser.add_argument("--release", action="store_true")
args = parser.parse_args()
is_release: bool = args.release


cef_path_raw = getenv("CEF_PATH")
if cef_path_raw is None:
    if os.path.exists(".cargo/config.toml"):
        with open(".cargo/config.toml", "rb") as f:
            cargo_config = tomllib.load(f)

        if "env" in cargo_config and "CEF_PATH" in cargo_config["env"]:
            maybe_cef_path = cargo_config["env"]["CEF_PATH"]

            if isinstance(maybe_cef_path, dict):
                if "value" in maybe_cef_path:
                    cef_path_raw = cast(str, maybe_cef_path["value"])
            else:
                cef_path_raw = maybe_cef_path

    if cef_path_raw is None:
        raise Exception(
            "環境変数`CEF_PATH`をcef-rsの`README.md`にある通りに設定してください。"
        )

cef_path = PurePath(cef_path_raw)

TARGET_DEBUG: Final = PurePath("target/debug")
BINARY_NAME: Final = "memex-app"
HELPER_BINARY_NAME: Final = "memex-cef-helper"
product_name: Final = "Memex Browser" if is_release else "Memex Browser (D)"
bundle_id: Final = "jp.tasuren.memex-poc" if is_release else "jp.tasuren.memex-poc-dev"
CEF_FRAMEWORK_NAME: Final = "Chromium Embedded Framework.framework"

logger.info("バンドルの作成: %s", product_name)

logger.debug("`Info.plist`テンプレートの読み込み")
with open("assets/helper_Info.plist", "r") as f:
    helper_info_plist_template = f.read()

logger.debug("ヘルパーの`Info.plist`テンプレートの読み込み")
with open("assets/Info.plist", "r") as f:
    info_plist_template = f.read()


def make_plist_template_kwargs(*, helper_type: str | None = None):
    """Info.plistのテンプレートに入れる値を作る。"""
    logger.debug("`Info.plist`テンプレートのkwargs作成: helper_type = %s", helper_type)
    app_name = product_name if helper_type is None else f"{product_name} {helper_type}"

    return {
        "region": "ja",
        "executable_name": app_name,
        "bundle_id": bundle_id if helper_type is None else f"{bundle_id}.helper",
        "product_name": app_name,
        "version_short": "0.1.0-alpha",
    }


# アプリのバンドルのフォルダを作りはじめる。
# 参考: https://bitbucket.org/chromiumembedded/cef/wiki/Tutorial?iframe=true&spa=0#markdown-header-mac-os-x-build-steps


def mkdir(path: PurePath) -> None:
    logger.debug("フォルダの準備: %s", path)

    if not os.path.exists(path):
        os.mkdir(path)


contents = TARGET_DEBUG / f"{product_name}.app" / "Contents"
logger.debug("メインバンドルの作成: %s", contents)

mkdir(contents.parent)
mkdir(contents)

frameworks = contents / "Frameworks"
macos = contents / "MacOS"
resources = contents / "Resources"

mkdir(frameworks)
mkdir(macos)
mkdir(resources)

logger.debug("メインバンドルの`Info.plist`を作成")
info_plist = info_plist_template.format(**make_plist_template_kwargs())

logger.debug("書き込み内容: %s", info_plist)
with open(contents / "Info.plist", "w") as f:
    f.write(info_plist)

logger.debug("CEFフレームワークを用意")
dst = frameworks / CEF_FRAMEWORK_NAME
if not os.path.exists(dst):
    shutil.copytree(cef_path / CEF_FRAMEWORK_NAME, dst)

symbols = cef_path / "symbols"
if os.path.exists(symbols) and os.path.isdir(symbols):
    logger.debug("デバッグシンボルがあったので、コピー")

    for file in os.listdir(symbols):
        if not file.endswith(".dSYM") or os.path.exists(dst / file):
            continue

        shutil.copytree(cef_path / "symbols" / file, dst / file)

# メインバンドルの構築
src = TARGET_DEBUG / BINARY_NAME
dst = macos / product_name
logger.debug("メインバンドルのバイナリを用意")
shutil.copy(src, dst)


# ヘルパーのバンドルを必要なもの全て作る。
def make_helper_bundle(type_: str, helper_executable_path: PurePath) -> None:
    """ヘルパーのバンドルを作る。

    Args:
        type_: ヘルパーの最後の名前。例：`Helper (GPU)`
        helper_executable_path: ヘルパーの実行ファイルのパス
    """
    logger.debug(
        "ヘルパーのバンドルの作成: helper_type = %s, helper_executable_path = %s",
        helper_type,
        helper_executable_path,
    )

    helper_name = f"{product_name} {type_}"
    helper_contents = frameworks / f"{helper_name}.app" / "Contents"
    helper_macos = helper_contents / "MacOS"

    logger.debug(
        "ヘルパーの設定: helper_name = %s, helper_contents = %s, helper_macos = %s",
        helper_name,
        helper_contents,
        helper_macos,
    )

    mkdir(helper_contents.parent)
    mkdir(helper_contents)
    mkdir(helper_macos)

    # 実行ファイルのコピー
    logger.debug("ヘルパーの実行ファイルを配置")
    shutil.copy(helper_executable_path, helper_macos / helper_name)

    logger.debug("ヘルパーの`Info.plist`の作成")
    info_plist = info_plist_template.format(
        **make_plist_template_kwargs(helper_type=type_)
    )

    with open(helper_contents / "Info.plist", "w") as f:
        f.write(info_plist)


HELPER_TYPES: Final = (
    "Helper (GPU)",
    "Helper (Renderer)",
    "Helper (Plugin)",
    "Helper (Alerts)",
    "Helper",
)

for helper_type in HELPER_TYPES:
    logger.info("ヘルパーの作成: %s", helper_type)
    make_helper_bundle(helper_type, TARGET_DEBUG / HELPER_BINARY_NAME)

logger.info("バンドルの作成終了")
