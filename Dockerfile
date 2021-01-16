FROM rust:1.49.0 as builder

WORKDIR ./

COPY . .

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian10

EXPOSE 8000

COPY --from=builder /usr/local/cargo/bin/rust_api /usr/local/bin/rust_api
COPY --from=builder key.pem key.pem
COPY --from=builder cert.pem cert.pem

CMD ["rust_api"]