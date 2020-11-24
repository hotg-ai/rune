eval $(ssh-agent)
ssh-add ~/.ssh/id_rsa

apt-get install libclang-dev

cargo build --release 