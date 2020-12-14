FROM rust:alpine

WORKDIR /usr/src/gladiators-player-service
COPY . .

RUN apk add --no-cache postgresql-client
RUN apk add --no-cache openssl
RUN apk add --no-cache openssl-dev
RUN apk add --no-cache musl-dev
RUN cargo install --path .

CMD ["gladiators-player-service"]
