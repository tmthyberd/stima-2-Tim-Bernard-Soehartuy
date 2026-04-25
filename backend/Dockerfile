FROM rust:1.80-slim as builder

WORKDIR /app
COPY . .

# Install dependencies needed for compiling reqwest and openssl
RUN apt-get update && apt-get install -y pkg-config libssl-dev

RUN cargo build --release

# Final runtime image
FROM debian:bookworm-slim

# Install libssl for runtime so reqwest works
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/backend .

EXPOSE 8080

CMD ["./backend"]
