#!/bin/env python

"""
A build script for generating and building custom versions of the
"rustembedded/cross" docker image for use in CI.
"""

import subprocess
from pathlib import Path

version = "0.2.1"

targets = [
    "x86_64-unknown-linux-gnu",
    "aarch64-unknown-linux-gnu",
]


def run(*args):
    command = " ".join(str(arg) for arg in args)
    print(f'Executing "{command}"')
    subprocess.run(args, check=True)


def dockerfile(target: str):
    return f"""
FROM rustembedded/cross:{target}-{version}

# Install the latest version of LLVM
RUN apt-get update && \
    apt-get install -y lsb-release wget software-properties-common apt-transport-https ca-certificates && \
    bash -c "$(curl https://apt.llvm.org/llvm.sh)"
"""


def build(target: str, filename: Path):
    run(
        "docker",
        "build",
        "-f",
        filename,
        f"--tag=tinyverseml/cross:{target}",
        f"--tag=tinyverseml/cross:{target}-{version}",
        ".",
    )


if __name__ == "__main__":
    util_dir = Path(__file__).parent

    for target in targets:
        filename = util_dir.joinpath(f"cross.{target}.Dockerfile")

        with open(filename, "w") as f:
            f.write(dockerfile(target))

        build(target, filename)

    run("docker", "push", "tinyverseml/cross", "--all-tags")
