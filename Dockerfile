FROM --platform=$BUILDPLATFORM tonistiigi/xx@sha256:0c6a569797744e45955f39d4f7538ac344bfb7ebf0a54006a0a4297b153ccf0f AS xx
ARG TARGETPLATFORM

FROM --platform=$BUILDPLATFORM rust:alpine@sha256:d57abe50ab0037f861817a85b4a26c8211d91075b5cca94a133ed02f803cd7c1 AS builder
RUN apk add clang lld openssl-dev
# copy xx scripts to your build stage
COPY --from=xx / /
ARG TARGETPLATFORM

RUN xx-apk add --no-cache musl-dev zlib-dev zlib-static openssl-dev openssl-libs-static pkgconfig alpine-sdk

WORKDIR /app
RUN cargo new --lib git-moves-together
WORKDIR /app/git-moves-together
COPY Cargo.* ./

RUN xx-cargo build --release --target-dir ./build
COPY . ./
RUN xx-cargo build --release --target-dir ./build && \
    xx-verify --static "./build/$(xx-cargo --print-target-triple)/release/git-moves-together" && \
    cp -v  "./build/$(xx-cargo --print-target-triple)/release/git-moves-together" "./build/git-moves-together"
RUN addgroup -g 568 nonroot
RUN adduser -u 568 -G nonroot -D nonroot
USER nonroot

FROM scratch
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

USER nonroot
COPY --from=builder /app/git-moves-together/build/git-moves-together .
RUN ["/git-moves-together", "--version"]
ENTRYPOINT ["/git-moves-together"]
