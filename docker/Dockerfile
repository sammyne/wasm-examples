FROM rust:1.77.2-slim-bookworm AS rust

ENV RUSTUP_DIST_SERVER="https://rsproxy.cn" \
  RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"

RUN sed -i 's/deb.debian.org/mirrors.tencent.com/g' /etc/apt/sources.list.d/debian.sources  &&\
  sed -i 's|security.debian.org/debian-security|mirrors.tencent.com/debian-security|g'      \
    /etc/apt/sources.list.d/debian.sources

# libssl-dev 和 pkg-config 是 cargo-component 的依赖
# clang 是 mold 链接器的依赖
RUN apt update                                  &&\
  apt install -y clang libssl-dev pkg-config    &&\
  apt clean                                     &&\
  rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

RUN rustup component add rustfmt && rustup target add wasm32-wasi


FROM rust AS cargo-bin

ARG CARGO_COMPONENT_VERSION=0.11.0  \
  WASM_TOOLS_VERSION=1.206.0

RUN cargo install cargo-component@$CARGO_COMPONENT_VERSION wasm-tools@$WASM_TOOLS_VERSION

WORKDIR /output

RUN cp `which cargo-component` `which wasm-tools` ./


FROM busybox:1.36.1 AS wasmtime

ARG WASMTIME_VERSION=20.0.0

RUN wget https://github.com/bytecodealliance/wasmtime/releases/download/v$WASMTIME_VERSION/wasmtime-v$WASMTIME_VERSION-x86_64-linux.tar.xz

RUN mkdir -p /output && tar -C /output --strip-components 1 -Jxvf wasmtime-v$WASMTIME_VERSION-x86_64-linux.tar.xz


FROM busybox:1.36.1 AS mold

ARG MOLD_VERSION=2.30.0

RUN wget https://github.com/rui314/mold/releases/download/v$MOLD_VERSION/mold-$MOLD_VERSION-x86_64-linux.tar.gz

RUN mkdir /output && tar -C /output --strip-components=1 -zxvf mold-$MOLD_VERSION-x86_64-linux.tar.gz


FROM rust

ENV PATH=/opt/wasmtime:/opt/mold/bin:$PATH

COPY --from=cargo-bin /output/* $CARGO_HOME/bin/

COPY --from=mold /output /opt/mold

COPY --from=wasmtime /output /opt/wasmtime
