#!/bin/sh

exec docker run -v `pwd`:`pwd` -w `pwd`  -i -t tinyverseml/rune-cli $@
