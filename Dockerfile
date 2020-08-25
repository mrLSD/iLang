FROM rust:1.45

RUN apt update && apt install -y cmake g++ pkg-config jq binutils-dev libcurl4-openssl-dev zlib1g-dev libdw-dev libiberty-dev
RUN cargo install cargo-kcov
RUN cargo kcov --print-install-kcov-sh > ./kcov-install.sh
RUN chmod u+x /kcov-install.sh
RUN ./kcov-install.sh
RUN rm ./kcov-install.sh

VOLUME /volume
WORKDIR /volume
