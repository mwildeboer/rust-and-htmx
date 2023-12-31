FROM rust:alpine

# Set the application directory
WORKDIR /app

# Install musl-tools to make many crates compile successfully
RUN apk add --no-cache musl-dev

# Install cargo-watch
RUN cargo install cargo-watch

# Copy the files to the Docker image
COPY ./ ./

CMD ["cargo", "watch", "-x", "run"]