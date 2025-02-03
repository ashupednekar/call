FROM rust:1.81 AS build-env

WORKDIR /backend


COPY . .

RUN cargo build --release -j 5

FROM gcr.io/distroless/cc-debian11

COPY --from=build-env /backend/target/release/call /backend/call
WORKDIR /backend

CMD ["./call"]
