FROM rust:1.80 AS builder
ARG TARGETPLATFORM

## Build deps for git-moves-together
RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y musl-tools pkg-config xutils-dev build-essential && \
    rm -vrf /var/lib/apt/lists/*

RUN groupadd -g 568 nonroot
RUN useradd -u 568 -g 568 nonroot
USER nonroot

RUN if [ "$TARGETPLATFORM" = "linux/amd64" ]; then  \
    rustup target add x86_64-unknown-linux-musl ;  \
    elif [ "$TARGETPLATFORM" = "linux/arm/v7" ]; then  \
    rustup target add armv7-unknown-linux-musleabihf ;  \
    elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then  \
    rustup target add aarch64-unknown-linux-musl ;  \
    else exit 1 ;  \
    fi

WORKDIR /app/git-moves-together
COPY . ./

ENV CC=musl-gcc
ENV CXX=musl-g++
ENV PKG_CONFIG_ALL_STATIC=true

RUN if [ "$TARGETPLATFORM" = "linux/amd64" ]; then  \
    cargo build --target=x86_64-unknown-linux-musl --release --features=vendored-libgit2-openssl ;  \
    elif [ "$TARGETPLATFORM" = "linux/arm/v7" ]; then  \
    cargo build --target=armv7-unknown-linux-musleabihf --release --features=vendored-libgit2-openssl ;  \
    elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then  \
    cargo build --target=aarch64-unknown-linux-musl --release --features=vendored-libgit2-openssl ;  \
    else exit 1 ;  \
    fi

FROM scratch
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

USER "nonroot"
COPY --from=builder /app/git-moves-together/target/*/release/git-moves-together .
RUN ["/git-moves-together", "--version"]
ENTRYPOINT ["/git-moves-together"]
