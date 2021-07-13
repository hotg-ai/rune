
    FROM rustembedded/cross:armv7-unknown-linux-gnueabihf-0.2.1

    # Install the latest version of LLVM
    RUN apt-get update &&         apt-get install -y lsb-release wget software-properties-common apt-transport-https ca-certificates &&         bash -c "$(curl https://apt.llvm.org/llvm.sh)"
    