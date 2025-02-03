FROM rust:1.81 AS build-env
WORKDIR /backend
ADD https://github.com/nats-io/nats-server/releases/download/v2.10.11/nats-server-v2.10.11-linux-amd64.tar.gz /tmp/
RUN tar -xvzf /tmp/nats-server-v2.10.11-linux-amd64.tar.gz -C /tmp \
    && mv /tmp/nats-server-v2.10.11-linux-amd64/nats-server /usr/local/bin/nats-server \
    && chmod +x /usr/local/bin/nats-server
COPY . .

RUN cargo build --release -j 5 
RUN chmod +x /backend/entrypoint.sh

FROM alpine 
COPY --from=build-env --chown=10014:10014 /usr/local/bin/nats-server /usr/local/bin/nats-server
COPY --from=build-env --chown=10014:10014 /backend/target/release/call /usr/local/bin/call
COPY --from=build-env --chown=10014:10014 /backend/entrypoint.sh /backend/entrypoint.sh
WORKDIR /backend
USER 10014

ENTRYPOINT ["/backend/entrypoint.sh"]
