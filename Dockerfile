FROM rust:1.80.1-alpine@sha256:1f5aff501e02c1384ec61bb47f89e3eebf60e287e6ed5d1c598077afc82e83d5 AS builder
ARG TARGETPLATFORM

RUN apk add --no-cache --purge \
      alpine-sdk \
      libc-dev \
      musl-dev \
      openssl-dev \
      openssl-libs-static \
      pkgconfig

RUN addgroup -g 568 nonroot
RUN adduser -u 568 -G nonroot -D nonroot
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

ENV PKG_CONFIG_ALL_STATIC=true

RUN if [ "$TARGETPLATFORM" = "linux/amd64" ]; then  \
    RUST_BACKTRACE=1 cargo build --target=x86_64-unknown-linux-musl --release ;  \
    elif [ "$TARGETPLATFORM" = "linux/arm/v7" ]; then  \
    RUST_BACKTRACE=1 cargo build --target=armv7-unknown-linux-musleabihf --release ;  \
    elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then  \
    RUST_BACKTRACE=1 cargo build --target=aarch64-unknown-linux-musl --release ;  \
    else exit 1 ;  \
    fi

FROM scratch
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

USER "nonroot"
COPY --from=builder /app/git-moves-together/target/*/release/git-moves-together .
RUN ["/git-moves-together", "--version"]
ENTRYPOINT ["/git-moves-together"]
