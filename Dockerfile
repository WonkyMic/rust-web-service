FROM rust:1 as builder
WORKDIR /app
COPY . .
RUN rustup default nightly
# RUN cargo build --release
RUN cargo install --path .

FROM debian:buster-slim as runner
COPY --from=builder /app/target/release ./
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["/wonkyapi.exe"]