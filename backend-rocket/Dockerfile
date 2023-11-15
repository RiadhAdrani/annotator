# Using official rust base image
FROM rust:alpine3.16

# Set the application directory
WORKDIR /app

EXPOSE 8000

# Install musl-tools to make many crates compile successfully
RUN apk add --no-cache musl-dev

# Install cargo-watch
RUN cargo install cargo-watch

# Copy the files to the Docker image
COPY ./ ./