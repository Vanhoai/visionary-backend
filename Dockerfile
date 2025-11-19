# Build Stage
FROM rust:bookworm AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

# Final Stage
FROM debian:bookworm-slim AS runner
WORKDIR /app
COPY --from=builder /app/target/release/application /app/application
CMD ["/app/application"]
