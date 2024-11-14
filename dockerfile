FROM rust AS build
RUN cargo new --bin j2mmd
WORKDIR /j2mmd
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs
COPY ./src ./src
RUN rm ./target/release/deps/j2mmd*
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=build /j2mmd/target/release/j2mmd .
ENTRYPOINT ["./j2mmd"]
