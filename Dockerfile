# Use the official Rust image as a build environment
FROM rust:latest as builder

# Install the musl target and necessary tools
RUN rustup target add x86_64-unknown-linux-musl && \
    apt-get update && apt-get install -y musl-tools pkg-config

# Set the working directory
WORKDIR /usr/src/app

# Copy the current directory contents into the container
COPY . .

# Build the application with static linking
RUN cargo build --release --target x86_64-unknown-linux-musl

# Use a minimal base image for the runtime (Alpine is lightweight and compatible with musl)
FROM alpine:latest

# Install minimal runtime dependencies (if needed)
RUN apk add --no-cache ca-certificates

# Copy the statically linked binary from the builder stage
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/booking-api /usr/local/bin/booking-api

# Expose port 9000 to the outside world
EXPOSE 7000

# Run the binary
CMD ["booking-api"]

