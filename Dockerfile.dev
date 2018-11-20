FROM ubuntu:18.04

ENV LANG C.UTF-8

RUN apt-get update -y

RUN apt-get update -y && apt-get install -y build-essential

RUN apt-get -y install curl

WORKDIR /app
ENV RUST_VERSION stable
ENV HOME /home
ENV USER $USER
ENV PATH $PATH:$HOME/.cargo/bin
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain ${RUST_VERSION} \
    && rustup component add rustfmt-preview \
    && rustup component add rls-preview rust-analysis rust-src \
    && rustup install stable

# llvmのインストール

RUN apt-get install -y wget \
    && wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key| apt-key add - \
    && echo "Acquire::http::No-Cache true;" >> /etc/apt/apt.conf \
    && echo "Acquire::http::Pipeline-Depth 0;" >> /etc/apt/apt.conf \
    &&apt-get install -y llvm-6.0

ENV LLVM_SYS_60_PREFIX=/usr/lib/llvm-6.0

RUN apt-get install -y zlib1g-dev

CMD ["cargo","run"]