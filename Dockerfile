FROM rust:1.56 AS build

WORKDIR /usr/src/lokaj-bot
COPY . .
RUN cargo fetch
RUN cargo install --path .

FROM debian:buster-slim AS run
RUN apt-get update && apt-get install -y libssl-dev ca-certificates libpq5 && update-ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=build /usr/local/cargo/bin/lokaj-bot /usr/local/bin/lokaj-bot

CMD ["lokaj-bot"]
