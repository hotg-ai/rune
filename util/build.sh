#!/bin/sh
set -xe
docker build . -t tinyverseml/rune-cli:latest -t tinyverseml/rune-cli:$(git rev-parse --short HEAD)
docker push tinyverseml/rune-cli --all-tags
