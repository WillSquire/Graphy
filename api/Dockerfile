# Todo: Add clippy linting check
# Todo: Add test runner check
# Todo: Add rustfmt style/format check

FROM rustlang/rust:nightly as build

# Build dependancies in new empty project (to cache)
RUN USER=root cargo new --bin api
WORKDIR /api
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release \
  && rm src/*.rs

# Build the project
COPY ./src ./src
COPY ./migrations ./migrations
RUN rm ./target/release/deps/api* \
  && cargo build --release --bins

# Copy binary to clean & minimal install
FROM rustlang/rust:nightly
COPY --from=build /api/target/release/api .
CMD ["./api"]
