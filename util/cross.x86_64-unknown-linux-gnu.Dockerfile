# A custom version of "rustembedded/cross" which includes clang so we can
# compile the "tflite" crate.
#
# Published using
#   docker build -f util/cross.x86_64-unknown-linux-gnu.Dockerfile -t tinyverseml/cross:x86_64-unknown-linux-gnu-0.2.1 -t tinyverseml/cross:x86_64-unknown-linux-gnu .
#   docker push tinyverseml/cross --all-tags
#
# See also: https://github.com/rust-embedded/cross#custom-docker-images
FROM rustembedded/cross:x86_64-unknown-linux-gnu-0.2.1

# Install the latest version of LLVM
RUN apt-get update && \
    apt-get install -y lsb-release wget software-properties-common apt-transport-https ca-certificates && \
    bash -c "$(curl https://apt.llvm.org/llvm.sh)"

