FROM rust:1.48.0 as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/saunadge-rs
COPY . .

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/local/cargo/bin/saunadge-rs /usr/local/bin/saunadge-rs

CMD ["saunadge-rs"]
