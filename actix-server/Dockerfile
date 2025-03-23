FROM rust:1.85-alpine AS builder
WORKDIR /app
COPY .. .
RUN apk add --no-cache musl-dev openssl-dev pkgconfig
RUN cargo build --release
RUN cargo install xh

# Create a minimal production image
FROM alpine:latest
WORKDIR /app
COPY --from=builder /app/target/release/rust1 .
COPY --from=builder /usr/local/cargo/bin/xh /bin/xh
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD xh --timeout=1 GET http://localhost:8080/healthy || exit 1
EXPOSE 8080
CMD ["./rust1"]