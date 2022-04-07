# Rust as the base image
FROM rust:1.59 as build

# Create a new empty shell project
RUN USER=root cargo new --bin rust-graphql-markdown-server
WORKDIR /rust-graphql-markdown-server

# Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY ./src ./src
COPY ./configs ./configs

# Build for release.
RUN rm ./target/release/deps/rust_graphql_markdown_server*
RUN cargo build --release

# The final base image
FROM debian:bullseye-slim

# Copy from the previous build
COPY --from=build /rust-graphql-markdown-server/configs /usr/src/configs
COPY --from=build /rust-graphql-markdown-server/target/release/rust-graphql-markdown-server /usr/src/rust-graphql-markdown-server
# COPY --from=build ["/rust-graphql-markdown-server/configs", "/rust-graphql-markdown-server/target/release/rust-graphql-markdown-server", "/usr/src/rust-graphql-markdown-server"]
# Run the binary
CMD RUN_ENV=DockerPro /usr/src/rust-graphql-markdown-server