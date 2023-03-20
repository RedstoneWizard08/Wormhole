import re
import argparse

files = [
    "cli/Cargo.toml",
    "gui/Cargo.toml",
    "common/Cargo.toml",
]


def bump_version(major: bool, minor: bool, patch: bool):
    bump_patch = 0
    bump_minor = 0
    bump_major = 0

    if major:
        bump_major = 1
        bump_minor = -1
        bump_patch = -1
    elif minor:
        bump_minor = 1
        bump_patch = -1
    elif patch:
        bump_patch = 1

    for file in files:
        with open(file, "r") as toml:
            content = str(toml.read())

            out = re.sub(
                '^version = "(\d+)\.(\d+)\.(\d+)"$',
                lambda exp:
                'version = "{}.{}.{}"'
                .format(
                    0 if bump_major == -1
                    else int(exp.groups()[0]) + bump_major,
                    0 if bump_minor == -1
                    else int(exp.groups()[1]) + bump_minor,
                    0 if bump_patch == -1
                    else int(exp.groups()[2]) + bump_patch
                ),
                content,
                flags=re.M
            )

            out = re.sub(
                '^wormhole-common = \{ path = "\.\.\/common", version = "(\d+)\.(\d+)\.(\d+)" \}$',
                lambda exp:
                'wormhole-common = { path = "../common", version = "' + "{}.{}.{}"
                .format(
                    0 if bump_major == -1
                    else int(exp.groups()[0]) + bump_major,
                    0 if bump_minor == -1
                    else int(exp.groups()[1]) + bump_minor,
                    0 if bump_patch == -1
                    else int(exp.groups()[2]) + bump_patch
                ) + '" }',
                out,
                flags=re.M
            )

            with open(file, "w") as write:
                write.write(out)


parser = argparse.ArgumentParser(
    prog="Wormhole Version Bumper",
    description="Bumps versions for Wormhole.",
)

parser.add_argument("-M", "--major", action="store_true",
                    help="Bump the major version.")
parser.add_argument("-m", "--minor", action="store_true",
                    help="Bump the minor version.")
parser.add_argument("-p", "--patch", action="store_true",
                    help="Bump the patch version.")

args = parser.parse_args()

bump_version(args.major, args.minor, args.patch)
