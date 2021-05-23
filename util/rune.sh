#! sh

docker run -v `pwd`:`pwd` -w `pwd`  -i -t tinyverseml/rune-cli /usr/local/bin/rune $@