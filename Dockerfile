# Builder Stage
FROM rust:1.59.0 AS builder
WORKDIR /app

# Install the required system dependencies
RUN apt update && apt install lld clang -y

# Set SQLX OFFLINE env var to true, this will force sqlx to use generated report for query verification
# instead of a live database connection
ENV SQLX_OFFLINE true

COPY . .
RUN cargo build --release

#===========================================

# Runtime stage
FROM debian:bullseye-slim AS runtime
WORKDIR /app

# Install OpenSSL - it is required by some dependencies
# Install ca-certificates - it is required to verify TLS Certificates
# when establishing HTTPS connection
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage to the runtime stage
COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration
ENV APP_ENVIRONMENT production

# Set entry point, launch the binary
ENTRYPOINT ["./zero2prod"]