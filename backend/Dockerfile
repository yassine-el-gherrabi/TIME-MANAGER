# Multi-stage Dockerfile for Rust backend
# Stage 1: Build
FROM rust:1.92-alpine AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev pkgconfig openssl-dev

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY migrations ./migrations

# Build for release
RUN cargo build --release

# Stage 2: Runtime
FROM alpine:3.19

# Install runtime dependencies
RUN apk add --no-cache libgcc openssl ca-certificates

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/timemanager-backend /app/timemanager-backend

# Copy migrations for runtime execution
COPY --from=builder /app/migrations /app/migrations

# Expose port
EXPOSE 8080

# Run the binary
CMD ["/app/timemanager-backend"]
