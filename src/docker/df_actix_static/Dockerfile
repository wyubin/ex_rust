FROM rust:alpine as builder
## install system need
RUN apk add --no-cache -U musl-dev
## compile
COPY /actix_static /tools/actix_static
RUN cargo install --path /tools/actix_static --root /tools
## bundle
FROM alpine
#FROM scratch
COPY --from=builder /tools/bin /tools
WORKDIR /tools
