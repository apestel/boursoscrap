#FROM rustdocker/rust:nightly as cargo-build
FROM clux/muslrust:stable as cargo-build
ENV PKG_CONFIG_ALLOW_CROSS=1
RUN apt-get update
RUN apt-get install musl-tools libssl-dev -y
RUN /root/.cargo/bin/rustup target add x86_64-unknown-linux-musl
RUN USER=root /root/.cargo/bin/cargo new --bin boursoscrap
WORKDIR /boursoscrap
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src
RUN RUSTFLAGS=-Clinker=musl-gcc /root/.cargo/bin/cargo build --release --target=x86_64-unknown-linux-musl
# RUN rm -rf target/x86_64-unknown-linux-musl	/release/deps/boursoscrap*
# RUN rm src/*.rs
# RUN RUSTFLAGS=-Clinker=musl-gcc /root/.cargo/bin/cargo build --release --target=x86_64-unknown-linux-musl
# RUN /root/.cargo/bin/cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine:latest
WORKDIR /app
COPY --from=cargo-build /boursoscrap/target/x86_64-unknown-linux-musl/release/boursoscrap /app/boursoscrap
EXPOSE 3000
CMD ["./boursoscrap"]