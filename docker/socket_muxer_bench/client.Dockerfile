### Build Image ###
FROM debian:stretch as builder

RUN echo "deb http://deb.debian.org/debian stretch-backports main" > /etc/apt/sources.list.d/backports.list && apt-get update && apt-get install -y protobuf-compiler/stretch-backports cmake golang curl && apt-get clean && rm -r /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain none
ENV PATH "$PATH:/root/.cargo/bin"

WORKDIR /xpeer
COPY rust-toolchain /xpeer/rust-toolchain
RUN rustup install $(cat rust-toolchain)

COPY . /xpeer
RUN cargo build --release -p network --bench socket_muxer_bench

### Production Image ###
FROM debian:stretch

RUN mkdir -p /opt/xpeer/bin /opt/xpeer/etc

# Hack to match bench binary since it doesn't output without the build hash
COPY --from=builder /xpeer/target/release/socket_muxer_bench-???????????????? /opt/xpeer/bin

ENV BENCH_PATTERN remote_tcp

CMD exec /opt/xpeer/bin/$(ls /opt/xpeer/bin | grep -e "^socket_muxer_bench-[0-9a-f]\{16\}$" | head -n 1) $BENCH_PATTERN

ARG BUILD_DATE
ARG GIT_REV
ARG GIT_UPSTREAM

LABEL org.label-schema.schema-version="1.0"
LABEL org.label-schema.build-date=$BUILD_DATE
LABEL org.label-schema.vcs-ref=$GIT_REV
LABEL vcs-upstream=$GIT_UPSTREAM
