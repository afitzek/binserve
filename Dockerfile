FROM rust:1.51 as build

WORKDIR /app
RUN USER=rust cargo init --lib .
COPY ./Cargo.lock .
COPY ./Cargo.toml .

RUN cargo build --lib --release

COPY ./src src
RUN touch src/main.rs

RUN cargo build --release

RUN strip /app/target/release/binserve

RUN echo "export PATH=/usr/local/cargo/bin:$PATH" >> /etc/profile

FROM gcr.io/distroless/cc-debian10

COPY --from=build /app/target/release/binserve /

ENTRYPOINT ["./binserve"]

